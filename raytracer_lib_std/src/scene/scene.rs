// TODO: use Vec<Light> if multiple light sources should be supported

use serde::Deserialize;
use serde::Serialize;

use raytracer_lib_no_std::Camera;

use crate::World;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BackendEnum {
    CpuSingleCore,
    CpuMultiCore,
    Cuda,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scene {
    world: World,
    camera: Camera,
    backend: BackendEnum,
}

pub trait SceneOps {
    fn new(world: World, camera: Camera, backend: BackendEnum) -> Scene;
}

impl SceneOps for Scene {
    fn new(world: World, camera: Camera, backend: BackendEnum) -> Scene {
        Scene {
            world,
            camera,
            backend,
        }
    }
}
