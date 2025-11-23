use crate::configuration::ConfigurationMessage;
use actix_web::web;
use actix_web::HttpResponse;
use std::sync::mpsc::Sender;

// curl -i -X PUT http://127.0.0.1:8090/api/camera/position/x/{x}
pub async fn camera_change_position_x(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_position_x = path_param.into_inner();
    match new_position_x.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraPositionX(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/camera/position/y/{y}
pub async fn camera_change_position_y(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_position_y = path_param.into_inner();
    match new_position_y.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraPositionY(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/camera/position/z/{z}
pub async fn camera_change_position_z(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_position_z = path_param.into_inner();
    match new_position_z.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraPositionZ(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/camera/fieldview/y/{y}
pub async fn camera_change_field_view_y(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_field_view_y = path_param.into_inner();
    match new_field_view_y.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraFieldViewY(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/camera/far/z/{z}
pub async fn camera_change_far_z(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_z_far = path_param.into_inner();
    match new_z_far.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraZFar(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/camera/target/x/{x}
pub async fn camera_change_target_x(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_target_x = path_param.into_inner();
    match new_target_x.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraTargetX(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/camera/target/y/{y}
pub async fn camera_change_target_y(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_target_y = path_param.into_inner();
    match new_target_y.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraTargetY(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/camera/target/z/{z}
pub async fn camera_change_target_z(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_target_z = path_param.into_inner();
    match new_target_z.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraTargetZ(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/camera/up/x/{x}
pub async fn camera_change_up_x(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_up_x = path_param.into_inner();
    match new_up_x.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraUpX(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/camera/up/y/{y}
pub async fn camera_change_up_y(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_up_y = path_param.into_inner();
    match new_up_y.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraUpY(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// curl -i -X PUT http://127.0.0.1:8090/api/camera/up/z/{z}
pub async fn camera_change_up_z(
    path_param: web::Path<String>,
    tx: web::Data<Sender<ConfigurationMessage>>,
) -> HttpResponse {
    let new_up_z = path_param.into_inner();
    match new_up_z.parse() {
        Ok(value) => {
            let _ = tx.send(ConfigurationMessage::CameraUpZ(value));
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
