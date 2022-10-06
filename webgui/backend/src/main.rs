#![feature(async_closure)]

use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::unbounded;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::info;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::reply::Json;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use raytracer_challenge_reference_impl::prelude::TileData;
use raytracer_challenge_reference_impl::prelude::{Camera, CameraOps, CanvasOpsStd};

use crate::index_html::INDEX_HTML;
use crate::scene::scene;

mod index_html;
mod scene;

#[derive(Deserialize, Serialize, Debug)]
struct WorldReq {
    name: String,
    rate: u32,
}

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let users = Users::default();
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());

    // GET /chat -> websocket upgrade
    let chat = warp::path("chat")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket, users))
        });

    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    let render = warp::post()
        .and(warp::path("render"))
        // Only accept bodies smaller than 64kb...
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::json())
        .map(post_render_scene());

    let routes = index.or(chat).or(render);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}

async fn user_connected(ws: WebSocket, users: Users) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, _) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, _) = mpsc::unbounded_channel();

    let width = 120;
    let height = 80;

    let (w, c) = scene(width, height);
    let width = c.get_hsize();
    let height = c.get_vsize();
    let (s, r) = unbounded::<Vec<TileData>>();

    tokio::task::spawn(async move {
        let start = Instant::now();
        Camera::render_multi_core_tile_producer(&c, &w, 5, 5, s);
        let dur = Instant::now() - start;
        info!("multi core duration: {:?}", dur);
    });

    tokio::task::spawn(async move {
        let mut cnt = 1;

        while let td = r.recv() {
            match td {
                Ok(tile_data) => {
                    println!("got  a tile  {}", cnt);
                    cnt += 1;

                    if cnt < 200 {
                        let tile_data_json = json!(tile_data);

                        let msg = Message::text(tile_data_json.to_string());
                        user_ws_tx
                            .send(msg)
                            .unwrap_or_else(|e| {
                                eprintln!("websocket send error: {}", e);
                            })
                            .await;
                    }
                }
                Err(e) => {
                    eprintln!("no more tiles available: {}", e);
                    break;
                }
            }
        }

        // let msg = Message::text(format!("image done"));
        // let _ = user_ws_tx.send(msg).unwrap_or_else(|e| {
        //     eprintln!("websocket send error: {}", e);
        // });

        // let msg = Message::text(format!("image done"));
        // user_ws_tx
        //     .send(msg)
        //     .unwrap_or_else(|e| {
        //         eprintln!("websocket send error: {}", e);
        //     })
        //     .await;
    });

    // Save the sender in our list of connected users.
    users.write().await.insert(my_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Every time the user sends a message, broadcast it to
    // all other users...
    // while let Some(result) = user_ws_rx.next().await {
    //     let msg = match result {
    //         Ok(msg) => {
    //             println!("sending message to other users {:?}", &msg);
    //             msg
    //         }
    //         Err(e) => {
    //             eprintln!("websocket error(uid={}): {}", my_id, e);
    //             break;
    //         }
    //     };
    //     user_message(my_id, msg, &users).await;
    // }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(my_id, &users).await;
}

async fn user_message(my_id: usize, msg: Message, users: &Users) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", my_id, msg);

    // New message from this user, send it to everyone else (except same uid)...
    for (&uid, tx) in users.read().await.iter() {
        if my_id != uid {
            if let Err(_disconnected) = tx.send(Message::text(new_msg.clone())) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}

async fn user_disconnected(my_id: usize, users: &Users) {
    eprintln!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    users.write().await.remove(&my_id);
}

fn post_render_scene() -> fn(WorldReq) -> Json {
    |world: WorldReq| {
        let width = 200;
        let height = 160;

        let (w, c) = scene(width, height);
        let width = c.get_hsize();
        let height = c.get_vsize();
        let (s, r) = unbounded::<Vec<TileData>>();

        thread::spawn(move || {
            info!("{}", format!("got a world   {:?}", &world));
            let start = Instant::now();
            Camera::render_multi_core_tile_producer(&c, &w, 5, 5, s);
            let dur = Instant::now() - start;
            info!("multi core duration: {:?}", dur);
        });

        let canvas = Camera::collect_tiles_to_canvas(r, width, height);

        let filename = "./chapter07_webgui_threaded.png";
        canvas.write_png(filename).expect("wrote png");
        info!("wrote file: {:?}", filename);

        let w = WorldReq {
            rate: 1234,
            name: "bumzack".to_string(),
        };
        warp::reply::json(&w)
    }
}
