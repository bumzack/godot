use std::thread;
use std::time::Instant;

use crossbeam_channel::unbounded;
use serde_derive::{Deserialize, Serialize};
use tracing_log::log::info;
use warp::Filter;

use raytracer_challenge_reference_impl::basics::TileData;
use raytracer_challenge_reference_impl::prelude::{Camera, CameraOps, CanvasOpsStd};

use crate::scene::scene;

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
        .map(|world: WorldReq| {
            let width = 2180;
            let height = 1800;

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
        });

    warp::serve(render).run(([127, 0, 0, 1], 3030)).await
}
