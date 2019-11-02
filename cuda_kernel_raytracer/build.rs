use ptx_builder::error::Result;
use ptx_builder::prelude::*;

fn main() -> Result<()> {
    //     println!("KERNEL_PTX_PATH_RUST_RENDER = {}", env!("KERNEL_PTX_PATH_RUST_RENDER"));
    CargoAdapter::with_env_var("KERNEL_PTX_PATH_RUST_RENDER").build(Builder::new(".")?)
}
