use std::time::Instant;

use serde_derive::{Deserialize, Serialize};
use tracing_log::log::info;
use warp::Filter;
use std::thread;
use raytracer_challenge_reference_impl::prelude::{Camera, CameraOps, Canvas, CanvasOpsStd};
use crossbeam_channel::unbounded;
use crate::scene::scene;
use raytracer_challenge_reference_impl::prelude::CanvasOps;
mod scene;

#[derive(Deserialize, Serialize, Debug)]
struct WorldReq {
    name: String,
    rate: u32,
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let render = warp::post()
        .and(warp::path("render"))
        // Only accept bodies smaller than 64kb...
        .and(warp::body::content_length_limit(1024 * 64))
        .and(warp::body::json())
        .map(|mut world: WorldReq| {
            let (w, c) = scene();

            let (s , r) = unbounded::<Canvas>();

            thread::spawn(move || {
                info!("{}",format!("got a world   {:?}", &world));
                let start = Instant::now();
                let canvas = Camera::render_multi_core_tiled_sender(&c, &w, 5, 5, s);
                let dur = Instant::now() - start;
                info!("multi core duration: {:?}", dur);
            });

          let full_image =   r.iter().reduce(|_, item| {
              println!("got something");
                info!("got a new image from render threads    {} {}", item.get_width(), item.get_height());
                item
            }).unwrap();


            let filename = "./chapter07_webgui_threaded.png";
            full_image.write_png(filename).expect("wrote png");
            info!("wrote file: {:?}",filename);


            let w = WorldReq {
                rate: 1234,
                name: "bumzack".to_string(),
            };
            warp::reply::json(&w)
        });

    warp::serve(render).run(([127, 0, 0, 1], 3030)).await
}