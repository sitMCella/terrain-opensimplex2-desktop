use crate::configuration::ConfigurationMessage;
use actix_web::web;
use actix_web::HttpResponse;
use std::sync::mpsc::Sender;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PositionX {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 30.05 }' -X PUT http://127.0.0.1:8090/api/camera/position/x
pub async fn camera_change_position_x(
    data: web::Json<PositionX>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_position_x = data.value;
    let _ = tx.send(ConfigurationMessage::CameraPositionX(new_position_x));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct PositionY {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 19.4 }' -X PUT http://127.0.0.1:8090/api/camera/position/y
pub async fn camera_change_position_y(
    data: web::Json<PositionY>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_position_y = data.value;
    let _ = tx.send(ConfigurationMessage::CameraPositionY(new_position_y));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct PositionZ {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 78.1 }' -X PUT http://127.0.0.1:8090/api/camera/position/z
pub async fn camera_change_position_z(
    data: web::Json<PositionZ>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_position_z = data.value;
    let _ = tx.send(ConfigurationMessage::CameraPositionZ(new_position_z));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct FieldViewY {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 41.11 }' -X PUT http://127.0.0.1:8090/api/camera/fieldview/y
pub async fn camera_change_field_view_y(
    data: web::Json<FieldViewY>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_field_view_y = data.value;
    let _ = tx.send(ConfigurationMessage::CameraFieldViewY(new_field_view_y));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct ZFar {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 50.98 }' -X PUT http://127.0.0.1:8090/api/camera/far/z
pub async fn camera_change_far_z(
    data: web::Json<ZFar>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_z_far = data.value;
    let _ = tx.send(ConfigurationMessage::CameraZFar(new_z_far));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct TargetX {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 16.35 }' -X PUT http://127.0.0.1:8090/api/camera/target/x
pub async fn camera_change_target_x(
    data: web::Json<TargetX>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_target_x = data.value;
    let _ = tx.send(ConfigurationMessage::CameraTargetX(new_target_x));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct TargetY {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": -5.81 }' -X PUT http://127.0.0.1:8090/api/camera/target/y
pub async fn camera_change_target_y(
    data: web::Json<TargetY>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_target_y = data.value;
    let _ = tx.send(ConfigurationMessage::CameraTargetY(new_target_y));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct TargetZ {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 4.23 }' -X PUT http://127.0.0.1:8090/api/camera/target/z
pub async fn camera_change_target_z(
    data: web::Json<TargetZ>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_target_z = data.value;
    let _ = tx.send(ConfigurationMessage::CameraTargetZ(new_target_z));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct UpX {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": -2.37 }' -X PUT http://127.0.0.1:8090/api/camera/up/x
pub async fn camera_change_up_x(
    data: web::Json<UpX>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_up_x = data.value;
    let _ = tx.send(ConfigurationMessage::CameraUpX(new_up_x));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct UpY {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 12.88 }' -X PUT http://127.0.0.1:8090/api/camera/up/y
pub async fn camera_change_up_y(
    data: web::Json<UpY>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_up_y = data.value;
    let _ = tx.send(ConfigurationMessage::CameraUpY(new_up_y));
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct UpZ {
    value: f32,
}

// curl -i -H "Content-Type: application/json" -d '{ "value": 2.3 }' -X PUT http://127.0.0.1:8090/api/camera/up/z
pub async fn camera_change_up_z(
    data: web::Json<UpZ>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_up_z = data.value;
    let _ = tx.send(ConfigurationMessage::CameraUpZ(new_up_z));
    HttpResponse::Ok().finish()
}
