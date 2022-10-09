use serde_derive::{Deserialize, Serialize};

use raytracer_challenge_reference_impl::example_scenes::chapter07::chapter07;
use raytracer_challenge_reference_impl::example_scenes::chapter15_smoothed_suzanne::chapter15_smoothed_suzanne;
use raytracer_challenge_reference_impl::prelude::{Camera, CameraOps, Tuple4D, World};

#[derive(Deserialize, Serialize, Debug)]
pub struct SceneConfig {
    id: usize,
    width: usize,
    height: usize,
    from: Tuple4D,
    to: Tuple4D,
    up: Tuple4D,
    antialiasing: usize,
    shadows: bool,
    fov: f64,
    name: String,
    size_area_light: usize,
}

impl SceneConfig {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_from(&self) -> Tuple4D {
        self.from
    }

    pub fn get_to(&self) -> Tuple4D {
        self.to
    }

    pub fn get_up(&self) -> Tuple4D {
        self.up
    }

    pub fn get_fov(&self) -> f64 {
        self.fov
    }

    pub fn get_size_area_light(&self) -> usize {
        self.size_area_light
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_antialiasing(&self) -> usize {
        self.antialiasing
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Scene {
    id: usize,
    filename: String,
    name: String,
    scene_data: SceneConfig,
    #[serde(skip_serializing, skip_deserializing)]
    c: Camera,
    #[serde(skip_serializing, skip_deserializing)]
    w: World,
}

impl Scene {
    pub fn new(id: usize, c: Camera, w: World, filename: String, name: String, scene_data: SceneConfig) -> Self {
        Scene {
            id,
            c,
            w,
            filename,
            name,
            scene_data,
        }
    }

    pub fn get_world(&self) -> &World {
        &self.w
    }

    pub fn get_camera(&self) -> &Camera {
        &self.c
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AllScenesDTO {
    scenes: Vec<Scene>,
}

impl AllScenesDTO {
    pub fn new() -> Self {
        AllScenesDTO { scenes: vec![] }
    }

    pub fn add(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn get_scenes(&self) -> &Vec<Scene> {
        &self.scenes
    }
}

pub fn get_scenes_dtos() -> AllScenesDTO {
    let mut result = AllScenesDTO::new();

    result.scenes.push(get_chapter07(1));
    result.scenes.push(get_suzanne_smoothed(2));

    result
}

fn get_chapter07(id: usize) -> Scene {
    let (w, c) = chapter07(100, 100);

    let scene_data = SceneConfig {
        id,
        width: 200,
        height: 160,
        from: *&c.clone().get_from(),
        to: *&c.clone().get_to(),
        up: *&c.clone().get_up(),
        antialiasing: 0,
        shadows: false,
        fov: c.get_field_of_view(),
        name: "chapter07".to_string(),
        size_area_light: 0,
    };

    let scene = Scene {
        id,
        filename: "chapter07_webui".to_string(),
        name: "chapter07".to_string(),
        scene_data,
        c,
        w,
    };
    scene
}

fn get_suzanne_smoothed(id: usize) -> Scene {
    let width = 1280;
    let height = 720;

    let pov = 0.7;
    let antialiasing = true;
    let antialiasing_size = 3;
    let arealight_u = 16;
    let arealight_v = 16;

    let (w, c) = chapter15_smoothed_suzanne(
        width,
        height,
        pov,
        antialiasing,
        antialiasing_size,
        arealight_u,
        arealight_v,
    );

    let scene_data = SceneConfig {
        id,
        width: 200,
        height: 160,
        from: *&c.clone().get_from(),
        to: *&c.clone().get_to(),
        up: *&c.clone().get_up(),
        antialiasing: 0,
        shadows: false,
        fov: c.get_field_of_view(),
        name: "suzanne_smoothed".to_string(),
        size_area_light: 16,
    };

    let scene = Scene {
        id,
        filename: "suzanne_smoothed_webui".to_string(),
        name: "suzanne_smoothed".to_string(),
        scene_data,
        c,
        w,
    };
    scene
}
