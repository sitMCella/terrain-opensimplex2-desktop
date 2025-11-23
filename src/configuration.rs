#[derive(Debug, Clone)]
pub enum ConfigurationMessage {
    TerrainWidth(f32),
    TerrainDepth(f32),
    TerrainSeed(i64),
    TerrainColor(String),
    CameraPositionX(f32),
    CameraPositionY(f32),
    CameraPositionZ(f32),
    CameraFieldViewY(f32),
    CameraZFar(f32),
    CameraTargetX(f32),
    CameraTargetY(f32),
    CameraTargetZ(f32),
    CameraUpX(f32),
    CameraUpY(f32),
    CameraUpZ(f32),
}
