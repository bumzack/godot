use std::time::Instant;

use serde_derive::{Deserialize, Serialize};

fn main() {
    let start = Instant::now();

    let w = 800;
    let h = 600;
    let color = 127;
    let p = Pixel::new(color, color, color, 255);
    let pixels = vec![p; w * h];
    let mut image = Image::new(w, h);
    image.set_pixels(pixels);

    let dur = Instant::now() - start;
    println!("image creation took : {:?}", dur);

    let start = Instant::now();
    let _ = serde_json::to_string(&image).unwrap();
    let dur = Instant::now() - start;
    println!("json serialization took : {:?}", dur);


    let bytes = w * h * 4;
    let kbytes = bytes as f32 / 1024.0;
    let mb = kbytes / 1024.0;
    let perf_mb_per_s = 200_f32;

    let expected = mb as f32 / perf_mb_per_s;
    let actual = dur.as_secs()  as f32 * mb  * 1000.0;

    println!("bytes {},  kb {} MB {} ", bytes, kbytes, mb);

    println!("expected performance {} ms, actual perf {:?} ms", expected/1000.0, dur.as_millis());

    main2();
    println!("expected performance {} MB/sec duration, actual perf {} MB/s ", perf_mb_per_s, actual);
}


fn main2() {
    let start = Instant::now();

    let w = 800;
    let h = 600;
    let color = 127;
    let pixels = vec![127_u8; w * h*4];
    let mut image = ImageVec::new(w, h);
    image.set_pixels(pixels);

    let dur = Instant::now() - start;
    println!("image creation took : {:?}", dur);

    let start = Instant::now();
    let _ = serde_json::to_string(&image).unwrap();
    let dur = Instant::now() - start;
    println!("json serialization took : {:?}", dur);


    let bytes = w * h * 4;
    let kbytes = bytes as f32 / 1024.0;
    let mb = kbytes / 1024.0;
    let perf_mb_per_s = 200_f32;

    let expected = mb as f32 / perf_mb_per_s;
    let actual = dur.as_secs()  as f32 * mb  * 1000.0;

    println!("bytes {},  kb {} MB {} ", bytes, kbytes, mb);

    println!("expected performance {} ms, actual perf {:?} ms", expected/1000.0, dur.as_millis());
    println!("expected performance {} MB/sec duration, actual perf {} MB/s ", perf_mb_per_s, actual);
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel {
            r,
            g,
            b,
            a,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![],
        }
    }

    pub fn set_pixels(&mut self, pixels: Vec<Pixel>) {
        self.pixels = pixels;
    }
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ImageVec {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl ImageVec {
    pub fn new(width: usize, height: usize) -> ImageVec {
        ImageVec {
            width,
            height,
            pixels: vec![],
        }
    }

    pub fn set_pixels(&mut self, pixels: Vec<u8>) {
        self.pixels = pixels;
    }
}

