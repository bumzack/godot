#![feature(async_closure)]

use std::ops::Add;
use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::{bounded, Sender};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use futures_util::task::SpawnExt;
use serde_json::json;
use tracing_log::log::error;
use warp::Filter;
use warp::ws::{Message, WebSocket};

use crate::index_html::INDEX_HTML;
use crate::structs::{Image, Pixel, SceneConfig};

mod index_html;
mod structs;


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

async fn render_scene(ws: WebSocket) {
    // let mut rng = rand::thread_rng();

    let (mut websocket_tx, mut websocket_rx) = ws.split();

    // wait for a message, which contains infos about the scene
    let w = websocket_rx.next().await.unwrap();

    match w {
        Ok(msg) => {
            println!("got a message  {:?}", msg.to_str());
            let scene_config: SceneConfig = serde_json::from_str(msg.to_str().unwrap()).unwrap();

            println!("scene config {:?}", &scene_config);

            // let recv_canvas = recv_web_sockets.clone();
            let (s, recv) = bounded::<String>(1);

            // start tokio task to render image
            tokio::task::spawn(async move {
                let start = Instant::now();
                render(scene_config, start, s);
                let dur = Instant::now() - start;
                println!("async renderer: {:?}", dur);
            });

            // start tokio task to send image via WebSocket
            tokio::task::spawn(async move {
                let start = Instant::now();
                loop {
                    let res_image = recv.recv();
                    match res_image {
                        Ok(image) => {
                            // let tile_data_json = json!(image).to_string();
                            let msg = Message::text(image);

                            websocket_tx
                                .send(msg)
                                .unwrap_or_else(|e| {
                                    error!("websocket send error: {}", e);
                                })
                                .await;
                        }
                        Err(e) => {
                            println!("no more tiles available: {}", e);
                            break;
                        }
                    }
                }
                let dur = Instant::now() - start;
                println!("async renderer: {:?}", dur);
            });
        }
        _ => {}
    }
}

fn render(scene_config: SceneConfig, start: Instant, sender: Sender<String>) {
    crossbeam::scope(|scope| {
        let mut children = vec![];


        for _i in 0..1 {
            let w = scene_config.get_width();
            let h = scene_config.get_height();
            let sender_thread = sender.clone();
            children.push(scope.spawn(move |_| {
                let tmp = Instant::now() - start.add(Duration::from_secs(1));

                let color: u8 = (tmp.as_micros() % 255) as u8;
                let color = 127;
                let p = Pixel::new(color, color, color, 255);
                let pixels = vec![p; w * h];
                let mut image = Image::new(w, h);
                image.set_pixels(pixels);

                let dur = Instant::now() - start;
                println!("image creation took : {:?}", dur);

                let image_json = serde_json::to_string(&image).unwrap();
                let dur = Instant::now() - start;
                println!("and json serialization took : {:?}", dur);



                match sender_thread.send(image_json) {
                    Ok(_) => {
                        println!("sending msg (=image) is success");
                    }
                    Err(e) => {
                        println!("error sending message {:?}", e);
                    }
                };

                thread::current().id()
            }));
        }

        for child in children {
            let dur = Instant::now() - start;
            let thread_id = child.join().unwrap();
            println!(
                "child thread {:?} finished. run for {:?}  ",
                thread_id, dur
            );
        }
        let dur = Instant::now() - start;
    })
        .expect("TODO: something went wrong");
}
