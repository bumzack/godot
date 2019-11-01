#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

use crate::{Canvas, World};
use raytracer_lib_no_std::Camera;
use std::time::Duration;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
pub struct Scene {
    world: World,
    camera: Camera,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "use_serde", derive(Deserialize, Serialize))]
pub struct RenderedScene {
    canvas: Canvas,
    duration: Duration,
}

pub trait SceneOps {
    fn new(world: World, camera: Camera) -> Scene;

    fn get_world(&self) -> &World;
    fn get_world_mut(&mut self) -> &mut World;
    fn get_camera(&self) -> &Camera;
}

impl SceneOps for Scene {
    fn new(world: World, camera: Camera) -> Scene {
        Scene { world, camera }
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
