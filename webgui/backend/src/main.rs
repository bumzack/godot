#![feature(async_closure)]

use std::time::Instant;

use crossbeam_channel::unbounded;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::error;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use raytracer_challenge_reference_impl::example_scenes::chapter15_smoothed_suzanne::chapter15_smoothed_suzanne;
use raytracer_challenge_reference_impl::example_scenes::test_soft_shadow_multiple_lights::test_soft_shadow_multiple_lights;
use raytracer_challenge_reference_impl::prelude::TileData;
use raytracer_challenge_reference_impl::prelude::{Camera, CameraOps, Canvas, CanvasOps, CanvasOpsStd};

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
    width: usize,
    height: usize,
}

impl WorldScene {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

async fn render_scene(ws: WebSocket) {
    let (mut websocket_tx, mut websocket_rx) = ws.split();

    // wait for a message, which contains infos about the scene
    let w = websocket_rx.next().await.unwrap();

    match w {
        Ok(world_tmp) => {
            let p: WorldScene = serde_json::from_str(world_tmp.to_str().unwrap()).unwrap();

            println!("worldscene {:?}", &p);

            let width = p.get_width();
            let height = p.get_height();
            let wi = p.get_width();
            let h = p.get_height();

            // let (w, c) = scene(width, height);

            //  let (w, c) = chapter15_smoothed_suzanne(width, height, 1.15, false, 3, 4, 4);
            let (w, c) = test_soft_shadow_multiple_lights(width, height, true, 3);
            let (s, recv_web_sockets) = unbounded::<TileData>();
            let recv_canvas = recv_web_sockets.clone();

            tokio::task::spawn(async move {
                let start = Instant::now();
                Camera::render_multi_core_tile_producer(&c, &w, 15, 15, s);
                let dur = Instant::now() - start;
                println!("async render_scene  multi core duration: {:?}", dur);
            });

            tokio::task::spawn(async move {
                let mut cnt = 1;
                let mut canvas = Canvas::new(wi, h);

                loop {
                    let td = recv_web_sockets.recv();
                    match td {
                        Ok(tile_data) => {
                            println!("warp backend got a tile idx {}", tile_data.get_idx());

                            tile_data.get_points().iter().for_each(|p| {
                                canvas.write_pixel(p.get_x(), p.get_y(), p.get_color());
                            });

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

                let filename = &format!("./webui_test_soft_shadow_multiple_lights_{}x{}.png", width, height,);
                canvas.write_png(filename).expect("write file");
            });

            // tokio::task::spawn(async move {
            //     let start = Instant::now();
            //     let canvas = Camera::collect_tiles_to_canvas(recv_canvas , width, height);
            //     let dur = Instant::now() - start;
            //     println!("async collect_tiles_to_canvas      duration: {:?}", dur);
            //     let filename = &format!(
            //         "./webui_test_soft_shadow_multiple_lights_{}x{}.png",
            //         width,
            //         height,
            //     );
            //     canvas.write_png(filename).expect("write file");
            // });
        }
        _ => {}
    }
}
