extern crate rustacuda;

use std::error::Error;
use std::ffi::CString;
use std::time::Instant;

use rustacuda::memory::{cuda_device_get_limit_stacksize, cuda_device_set_limit_stacksize, DeviceBox};
use rustacuda::prelude::{
    Context, ContextFlags, CopyDestination, CudaFlags, Device, DeviceBuffer, Module, Stream, StreamFlags,
};

use crate::backend::backend::Backend;
use raytracer_lib_no_std::{Camera, CameraOps, ColorOps, BLACK};
use raytracer_lib_std::{Canvas, CanvasOps, World, WorldOps};

pub struct BackendCuda {}

impl Backend for BackendCuda {
    fn render_world(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>> {
        let start = Instant::now();

        // CUDA setup
        rustacuda::init(CudaFlags::empty())?;

        let device = Device::get_device(0)?;

        let _context = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

        // Load the module containing the function we want to call
        // let ptx = env!("KERNEL_PTX_PATH_RUST_RENDER");
        // println!("ptx = {}", ptx);
        // let ptx_content = include_str!(ptx);
        //    let module_data = CString::new(ptx_content)?;
        let module_data = CString::new(include_str!("/tmp/ptx-builder-0.5/cuda_kernel_raytracer/dbaccfb949de4deb/nvptx64-nvidia-cuda/release/cuda_kernel_raytracer.ptx")).expect("Unable to create sources");
        let module = Module::load_from_string(&module_data).expect("Unable to create kernel name string");

        //  Create a stream to submit work to
        let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

        let a = unsafe { cuda_device_get_limit_stacksize() }?;
        // println!(" cudaLimitSTackSize   = {}", a);
        let a = a * 40;
        // println!(" set stack size to 20x the size   = {}", a);
        let _b = unsafe { cuda_device_set_limit_stacksize(a) };

        let _a = unsafe { cuda_device_get_limit_stacksize() }?;
        // println!(" new  stack size    = {}", a);

        // width and height
        let w = c.get_hsize();
        let h = c.get_vsize();
        let mut width = DeviceBox::new(&(w as f32)).expect("DeviceBox::new(w)   image save expect in 'backend_cuda' ");
        let mut height = DeviceBox::new(&(h as f32)).expect("DeviceBox::new(h)   image save expect in 'backend_cuda' ");

        // PIXELS
        let mut pixels_vec = vec![BLACK; c.get_vsize() as usize * c.get_hsize() as usize];
        let mut pixels = DeviceBuffer::from_slice(&pixels_vec)
            .expect("DeviceBuffer::from_slice(&pixels_vec)    image save expect in 'backend_cuda' ");

        let mut shapes_device = DeviceBuffer::from_slice(world.get_shapes_mut())
            .expect("DeviceBuffer::from_slice(&shapes)    image save expect in 'backend_cuda' ");
        let cnt_shapes = world.get_shapes().len();

        // we are using a vec of lights, world has only 1 light
        let mut lights_vec = Vec::new();
        lights_vec.push(world.get_light().clone());
        let mut lights_device = DeviceBuffer::from_slice(&lights_vec)
            .expect("DeviceBuffer::from_slice(&lights)    image save expect in 'backend_cuda' ");
        let cnt_lights = lights_vec.len();

        // CAMERA
        let camera_clone = c.clone();
        let mut camera_device =
            DeviceBox::new(&camera_clone).expect("DeviceBox::new(camera_clone)   image save expect in 'backend_cuda' ");

        // CUDA setup block/grid
        let b = (256, 1, 1);
        let block = (b.0 as u32, b.1 as u32, b.2 as u32);

        let g = (
            (w as i32 + block.0 as i32 - 1) / block.0 as i32,
            (h as i32 + block.1 as i32 - 1) / block.1 as i32,
            1 as i32,
        );
        let grid = (g.0 as u32, g.1 as u32, 1 as u32);
        // println!("block = {:?}, grid = {:?}", block, grid);

        unsafe {
            launch!(module.calc_pixel<<<grid, block, 0, stream>>>(
                pixels.as_device_ptr(),
                shapes_device.as_device_ptr(),
                cnt_shapes,
                lights_device.as_device_ptr(),
                cnt_lights,
                camera_device.as_device_ptr(),
                width.as_device_ptr(),
                height.as_device_ptr(),
                block.0,
                block.1
            ))?;
        }
        stream
            .synchronize()
            .expect("----     stream.synchronize()       expect in 'backend_cuda' ");

        pixels
            .copy_to(&mut pixels_vec)
            .expect(" pixels.copy_to(&mut pixels_vec)             expect in 'backend_cuda' ");

        let stopped = Instant::now();
        println!("\ncuda   {:?} \n", stopped.duration_since(start));

        // TODO: is there a easier way, than to iterate over all pixels ?
        let mut c = Canvas::new(w, h);

        let mut x = 0;
        let mut y = 0;
        let mut idx = 0;
        for p in pixels_vec.iter_mut() {
            // println!("pixels_vec = {:?}, pixel = {:?}", p, pixel);
            p.clamp_color();
            c.write_pixel(x, y, *p);
            x = x + 1;
            idx = idx + 1;
            if x % w == 0 {
                y = y + 1;
                x = 0;
            }
        }
        Ok(c)
    }

    fn render_world_multi_core(&self, world: &mut World, c: &Camera) -> Result<Canvas, Box<dyn Error>> {
        self.render_world(world, c)
    }
}

impl BackendCuda {
    pub fn new() -> BackendCuda {
        BackendCuda {}
    }
}
