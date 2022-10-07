#![feature(async_closure)]

use std::time::Instant;

use crossbeam_channel::unbounded;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::error;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use raytracer_challenge_reference_impl::prelude::TileData;
use raytracer_challenge_reference_impl::prelude::{Camera, CameraOps};

use crate::index_html::INDEX_HTML;
use crate::scene::scene;

mod index_html;
mod scene;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // GET /chat -> websocket upgrade
    let render = warp::path("openwebsocket")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| render_scene(socket))
        });

    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    let routes = index.or(render);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}

#[derive(Deserialize, Serialize, Debug)]
struct WorldScene {
    pub width: usize,
    pub height: usize,
}

async fn render_scene(ws: WebSocket) {
    let (mut websocket_tx, mut websocket_rx) = ws.split();

    // wait for a message, which contains infos about the scene
    let w = websocket_rx.next().await.unwrap();

    match w {
        Ok(world_tmp) => {
            let p: WorldScene = serde_json::from_str(world_tmp.to_str().unwrap()).unwrap();

            println!("worldscene {:?}", &p);

            let width = p.width;
            let height = p.height;

            let (w, c) = scene(width, height);
            let (s, r) = unbounded::<TileData>();

            tokio::task::spawn(async move {
                let start = Instant::now();
                Camera::render_multi_core_tile_producer(&c, &w, 5, 5, s);
                let dur = Instant::now() - start;
                println!("async render_scene  multi core duration: {:?}", dur);
            });

            tokio::task::spawn(async move {
                let mut cnt = 1;

                loop {
                    let td = r.recv();
                    match td {
                        Ok(tile_data) => {
                            println!("got  a tile  {}", cnt);

                            let tile_data_json = json!(tile_data).to_string();
                            let msg = Message::text(tile_data_json);

                            websocket_tx
                                .send(msg)
                                .unwrap_or_else(|e| {
                                    error!("websocket send error: {}", e);
                                })
                                .await;
                            cnt += 1;
                        }
                        Err(e) => {
                            println!("no more tiles available: {}", e);
                            break;
                        }
                    }
                }
            });
        }
        _ => {}
    }
}
