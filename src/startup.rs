use crate::configuration::ConfigurationMessage;
use crate::routes::{
    camera_change_far_z, camera_change_field_view_y, camera_change_position_x,
    camera_change_position_y, camera_change_position_z, camera_change_target_x,
    camera_change_target_y, camera_change_target_z, camera_change_up_x, camera_change_up_y,
    camera_change_up_z, health_check, terrain_change_color, terrain_change_depth,
    terrain_change_failoff, terrain_change_fractal_frequency, terrain_change_fractal_octaves,
    terrain_change_max_height, terrain_change_seed, terrain_change_width, terrain_change_z,
};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::web::ServiceConfig;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::mpsc::Sender;

const PORT: i32 = 8090;

pub async fn start_server(tx: Sender<ConfigurationMessage>) -> std::io::Result<()> {
    println!("Tokio running on port {}", PORT);
    let address = format!("0.0.0.0:{}", PORT);
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    run(listener, tx)?.await
}

fn run(listener: TcpListener, tx: Sender<ConfigurationMessage>) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new().wrap(cors).configure(config_app(tx.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

fn config_app(tx: Sender<ConfigurationMessage>) -> Box<dyn Fn(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(tx.clone()))
            .service(web::resource("/api/health_check").route(web::get().to(health_check)))
            .service(
                web::resource("/api/terrain/width/{width}")
                    .route(web::put().to(terrain_change_width)),
            )
            .service(
                web::resource("/api/terrain/depth/{depth}")
                    .route(web::put().to(terrain_change_depth)),
            )
            .service(
                web::resource("/api/terrain/seed/{seed}").route(web::put().to(terrain_change_seed)),
            )
            .service(
                web::resource("/api/terrain/color/{color}")
                    .route(web::put().to(terrain_change_color)),
            )
            .service(
                web::resource("/api/terrain/height/{height}")
                    .route(web::put().to(terrain_change_max_height)),
            )
            .service(
                web::resource("/api/terrain/failoff/{failoff}")
                    .route(web::put().to(terrain_change_failoff)),
            )
            .service(web::resource("/api/terrain/z/{z}").route(web::put().to(terrain_change_z)))
            .service(
                web::resource("/api/terrain/fractal/octaves/{octaves}")
                    .route(web::put().to(terrain_change_fractal_octaves)),
            )
            .service(
                web::resource("/api/terrain/fractal/frequency/{frequency}")
                    .route(web::put().to(terrain_change_fractal_frequency)),
            )
            .service(
                web::resource("/api/camera/position/x/{x}")
                    .route(web::put().to(camera_change_position_x)),
            )
            .service(
                web::resource("/api/camera/position/y/{y}")
                    .route(web::put().to(camera_change_position_y)),
            )
            .service(
                web::resource("/api/camera/position/z/{z}")
                    .route(web::put().to(camera_change_position_z)),
            )
            .service(
                web::resource("/api/camera/fieldview/y/{y}")
                    .route(web::put().to(camera_change_field_view_y)),
            )
            .service(
                web::resource("/api/camera/far/z/{z}").route(web::put().to(camera_change_far_z)),
            )
            .service(
                web::resource("/api/camera/target/x/{x}")
                    .route(web::put().to(camera_change_target_x)),
            )
            .service(
                web::resource("/api/camera/target/y/{y}")
                    .route(web::put().to(camera_change_target_y)),
            )
            .service(
                web::resource("/api/camera/target/z/{z}")
                    .route(web::put().to(camera_change_target_z)),
            )
            .service(web::resource("/api/camera/up/x/{x}").route(web::put().to(camera_change_up_x)))
            .service(web::resource("/api/camera/up/y/{y}").route(web::put().to(camera_change_up_y)))
            .service(
                web::resource("/api/camera/up/z/{z}").route(web::put().to(camera_change_up_z)),
            );
    })
}
