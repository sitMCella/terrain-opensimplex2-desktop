use crate::configuration::ConfigurationMessage;
use three_d::*;

#[derive(Debug, Clone, Copy)]
pub struct CameraConfiguration {
    position_x: f32,
    position_y: f32,
    position_z: f32,
    field_view_y: f32,
    z_far: f32,
    target_x: f32,
    target_y: f32,
    target_z: f32,
    up_x: f32,
    up_y: f32,
    up_z: f32,
}

impl CameraConfiguration {
    pub fn new(
        position_x: f32,
        position_y: f32,
        position_z: f32,
        field_view_y: f32,
        z_far: f32,
        target_x: f32,
        target_y: f32,
        target_z: f32,
        up_x: f32,
        up_y: f32,
        up_z: f32,
    ) -> Self {
        Self {
            position_x,
            position_y,
            position_z,
            field_view_y,
            z_far,
            target_x,
            target_y,
            target_z,
            up_x,
            up_y,
            up_z,
        }
    }
}

pub fn configure_camera(camera_configuration: &CameraConfiguration) -> Camera {
    Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        vec3(
            camera_configuration.position_x,
            camera_configuration.position_y,
            camera_configuration.position_z,
        ),
        vec3(
            camera_configuration.target_x,
            camera_configuration.target_y,
            camera_configuration.target_z,
        ),
        vec3(
            camera_configuration.up_x,
            camera_configuration.up_y,
            camera_configuration.up_z,
        ),
        degrees(camera_configuration.field_view_y),
        0.1,
        camera_configuration.z_far,
    )
}

pub fn update_configuration(
    camera_configuration: CameraConfiguration,
    msg: Option<ConfigurationMessage>,
) -> CameraConfiguration {
    match msg {
        Some(ConfigurationMessage::CameraPositionX(value)) => CameraConfiguration {
            position_x: value,
            ..camera_configuration
        },
        Some(ConfigurationMessage::CameraPositionY(value)) => CameraConfiguration {
            position_y: value,
            ..camera_configuration
        },
        Some(ConfigurationMessage::CameraPositionZ(value)) => CameraConfiguration {
            position_z: value,
            ..camera_configuration
        },
        Some(ConfigurationMessage::CameraFieldViewY(value)) => CameraConfiguration {
            field_view_y: value,
            ..camera_configuration
        },
        Some(ConfigurationMessage::CameraZFar(value)) => CameraConfiguration {
            z_far: value,
            ..camera_configuration
        },
        Some(ConfigurationMessage::CameraTargetX(value)) => CameraConfiguration {
            target_x: value,
            ..camera_configuration
        },
        Some(ConfigurationMessage::CameraTargetY(value)) => CameraConfiguration {
            target_y: value,
            ..camera_configuration
        },
        Some(ConfigurationMessage::CameraTargetZ(value)) => CameraConfiguration {
            target_z: value,
            ..camera_configuration
        },
        Some(ConfigurationMessage::CameraUpX(value)) => CameraConfiguration {
            up_x: value,
            ..camera_configuration
        },
        Some(ConfigurationMessage::CameraUpY(value)) => CameraConfiguration {
            up_y: value,
            ..camera_configuration
        },
        Some(ConfigurationMessage::CameraUpZ(value)) => CameraConfiguration {
            up_z: value,
            ..camera_configuration
        },
        None => camera_configuration.clone(),
        _ => camera_configuration.clone(),
    }
}
