use std::sync::mpsc;
use std::thread;
use terrainopensimplex2::configuration::ConfigurationMessage;
use terrainopensimplex2::startup::start_server;
use terrainopensimplex2::visualization::window;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<ConfigurationMessage>();

    thread::spawn(move || {
        actix_web::rt::System::new().block_on(async {
            start_server(tx).await.unwrap();
        });
    });

    window(rx);
}
