use crate::configuration::ConfigurationMessage;
use actix_web::web;
use actix_web::HttpResponse;
use std::sync::mpsc::Sender;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Width {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 15.75 }' -X PUT http://127.0.0.1:8090/api/terrain/width
pub async fn terrain_change_width(
    data: web::Json<Width>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_width = data.value;
    let _ = tx.send(ConfigurationMessage::TerrainWidth(new_width));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct Depth {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 9.25 }' -X PUT http://127.0.0.1:8090/api/terrain/depth
pub async fn terrain_change_depth(
    data: web::Json<Depth>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_depth = data.value;
    let _ = tx.send(ConfigurationMessage::TerrainDepth(new_depth));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct Seed {
    value: i64,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 42 }' -X PUT http://127.0.0.1:8090/api/terrain/seed
pub async fn terrain_change_seed(
    data: web::Json<Seed>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_seed = data.value;
    let _ = tx.send(ConfigurationMessage::TerrainSeed(new_seed));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct CubeSize {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 0.2 }' -X PUT http://127.0.0.1:8090/api/terrain/cubesize
pub async fn terrain_change_cube_size(
    data: web::Json<CubeSize>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_cube_size = data.value;
    println!("new_cube_size {}", new_cube_size);
    if new_cube_size <= 0.0f32 {
        return HttpResponse::BadRequest().finish();
    }
    let _ = tx.send(ConfigurationMessage::TerrainCubeSize(new_cube_size));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct Color {
    value: String,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": "4955ff" }' -X PUT http://127.0.0.1:8090/api/terrain/color
pub async fn terrain_change_color(
    data: web::Json<Color>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_color = &data.value;
    if new_color.len() != 6 {
        return HttpResponse::BadRequest().finish();
    }

    let _ = match u8::from_str_radix(&new_color[0..2], 16) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let _ = match u8::from_str_radix(&new_color[2..4], 16) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let _ = match u8::from_str_radix(&new_color[4..6], 16) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let _ = tx.send(ConfigurationMessage::TerrainColor(new_color.clone()));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct MaxHeight {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 5.22 }' -X PUT http://127.0.0.1:8090/api/terrain/height
pub async fn terrain_change_max_height(
    data: web::Json<MaxHeight>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_max_height = data.value;
    let _ = tx.send(ConfigurationMessage::TerrainMaxHeight(new_max_height));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct Failoff {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 95.48 }' -X PUT http://127.0.0.1:8090/api/terrain/failoff
pub async fn terrain_change_failoff(
    data: web::Json<Failoff>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_failoff = data.value;
    let _ = tx.send(ConfigurationMessage::TerrainFailoff(new_failoff));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct Z {
    value: f64,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 43.11 }' -X PUT http://127.0.0.1:8090/api/terrain/z
pub async fn terrain_change_z(
    data: web::Json<Z>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_z = data.value;
    let _ = tx.send(ConfigurationMessage::TerrainZ(new_z));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct Octaves {
    value: i32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 8 }' -X PUT http://127.0.0.1:8090/api/terrain/fractal/octaves
pub async fn terrain_change_fractal_octaves(
    data: web::Json<Octaves>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_fractal_octaves = data.value;
    let _ = tx.send(ConfigurationMessage::TerrainFractalOctaves(new_fractal_octaves));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct FractalAmplitude {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 0.25 }' -X PUT http://127.0.0.1:8090/api/terrain/fractal/amplitude
pub async fn terrain_change_fractal_amplitude(
    data: web::Json<FractalAmplitude>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_fractal_amplitude = data.value;
    let _ = tx.send(ConfigurationMessage::TerrainFractalAmplitude(new_fractal_amplitude));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct FractalFrequency {
    value: f64,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 6.99 }' -X PUT http://127.0.0.1:8090/api/terrain/fractal/frequency
pub async fn terrain_change_fractal_frequency(
    data: web::Json<FractalFrequency>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_fractal_frequency = data.value;
    let _ = tx.send(ConfigurationMessage::TerrainFractalFrequency(new_fractal_frequency));
    HttpResponse::Ok().finish()
}
