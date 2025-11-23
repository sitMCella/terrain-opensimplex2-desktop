use crate::configuration::ConfigurationMessage;
use actix_web::web;
use actix_web::HttpResponse;
use std::sync::mpsc::Sender;

// curl -i -X PUT http://127.0.0.1:8090/api/terrain/width/{width}
pub async fn terrain_change_width(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_width = path_param.into_inner();
    match new_width.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::TerrainWidth(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/terrain/depth/{depth}
pub async fn terrain_change_depth(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_depth = path_param.into_inner();
    match new_depth.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::TerrainDepth(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/terrain/seed/{seed}
pub async fn terrain_change_seed(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_seed = path_param.into_inner();
    match new_seed.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::TerrainSeed(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/terrain/color/{color}
pub async fn terrain_change_color(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_color = path_param.into_inner();
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

    let _ = tx.send(ConfigurationMessage::TerrainColor(new_color));
    HttpResponse::Ok().finish()
}

// curl -i -X PUT http://127.0.0.1:8090/api/terrain/height/{height}
pub async fn terrain_change_max_height(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_max_height = path_param.into_inner();
    match new_max_height.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::TerrainMaxHeight(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/terrain/fractal/octaves/{octaves}
pub async fn terrain_change_fractal_octaves(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_fractal_octaves = path_param.into_inner();
    match new_fractal_octaves.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::TerrainFractalOctaves(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
