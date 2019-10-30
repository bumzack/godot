use std::time::Duration;

use serde::Deserialize;
use serde::Serialize;

use raytracer_lib_no_std::Camera;

use crate::{Canvas, World};

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RenderedScene {
    canvas: Canvas,
    duration: Duration,
}

pub trait SceneOps {
    fn new(world: World, camera: Camera, backend: BackendEnum) -> Scene;

    fn get_backend(&self) -> &BackendEnum;
    fn get_world(&self) -> &World;
    fn get_world_mut(&mut self) -> &mut World;
    fn get_camera(&self) -> &Camera;
}

impl SceneOps for Scene {
    fn new(world: World, camera: Camera, backend: BackendEnum) -> Scene {
        Scene { world, camera, backend }
    }

    fn get_backend(&self) -> &BackendEnum {
        &self.backend
    }

    fn get_world(&self) -> &World {
        &self.world
    }
    fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn get_camera(&self) -> &Camera {
        &self.camera
    }
}

pub trait RenderedSceneOps {
    fn new(canvas: Canvas, duration: Duration) -> RenderedScene;

    fn get_canvas(&self) -> &Canvas;
    fn get_duration(&self) -> &Duration;
}

impl RenderedSceneOps for RenderedScene {
    fn new(canvas: Canvas, duration: Duration) -> RenderedScene {
        RenderedScene { canvas, duration }
    }

    fn get_canvas(&self) -> &Canvas {
        &self.canvas
    }

    fn get_duration(&self) -> &Duration {
        &self.duration
    }
}
