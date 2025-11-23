use crate::camera_configuration::{configure_camera, CameraConfiguration};
use crate::configuration::ConfigurationMessage;
use crate::terrain_configuration::{configure_terrain, TerrainConfiguration};
use std::sync::mpsc::Receiver;
use three_d::WindowedContext;
use three_d::*;

pub fn window(rx: Receiver<ConfigurationMessage>) {
    let event_loop = winit::event_loop::EventLoop::new();

    #[cfg(not(target_arch = "wasm32"))]
    let window_builder = winit::window::WindowBuilder::new()
        .with_title("Terrain Visualization")
        .with_min_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .with_maximized(true);
    #[cfg(target_arch = "wasm32")]
    let window_builder = {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        winit::window::WindowBuilder::new()
            .with_canvas(Some(
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_elements_by_tag_name("canvas")
                    .item(0)
                    .unwrap()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap(),
            ))
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .with_prevent_default(true)
    };
    let window = window_builder.build(&event_loop).unwrap();
    let context = WindowedContext::from_winit_window(&window, SurfaceSettings::default()).unwrap();

    let mut camera_configuration = CameraConfiguration::new(
        35.0, 22.0, 82.0, 45.0, 100.0, 18.0, -10.0, 0.0, 0.0, 15.0, 0.0,
    );

    let mut camera = configure_camera(&camera_configuration);
    let mut control = OrbitControl::new(camera.target(), 1.0, 100.0);

    let mut terrain_configuration =
        TerrainConfiguration::new(50.0, 50.0, 40000345266, "304630".to_string(), 4.0, 200.0, 25.0, 3, 2.0);

    let mut model = configure_terrain(&context, &terrain_configuration);

    let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            let mut frame_input = frame_input_generator.generate(&context);

            control.handle_events(&mut camera, &mut frame_input.events);
            camera.set_viewport(frame_input.viewport);
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .render(&camera, &model, &[]);

            context.swap_buffers().unwrap();
            control_flow.set_poll();
            window.request_redraw();
        }
        winit::event::Event::WindowEvent { ref event, .. } => {
            frame_input_generator.handle_winit_window_event(event);
            match event {
                winit::event::WindowEvent::Resized(physical_size) => {
                    context.resize(*physical_size);
                }
                winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    context.resize(**new_inner_size);
                }
                winit::event::WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                _ => (),
            }
        }
        _ => {
            while let Ok(msg) = rx.try_recv() {
                (terrain_configuration, camera_configuration) = update_configuration(
                    terrain_configuration.clone(),
                    camera_configuration.clone(),
                    Some(msg),
                );
                model = configure_terrain(&context, &terrain_configuration);
                camera = configure_camera(&camera_configuration);
                window.request_redraw();
            }
        }
    });
}

fn update_configuration(
    terrain_configuration: TerrainConfiguration,
    camera_configuration: CameraConfiguration,
    msg: Option<ConfigurationMessage>,
) -> (TerrainConfiguration, CameraConfiguration) {
    (
        crate::terrain_configuration::update_configuration(terrain_configuration, msg.clone()),
        crate::camera_configuration::update_configuration(camera_configuration, msg.clone()),
    )
}
