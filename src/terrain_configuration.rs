use crate::configuration::ConfigurationMessage;
use crate::smooth::noise3_ImproveXZ;
use three_d::*;

const CUBE_SIZE: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct TerrainConfiguration {
    tot_width: f32,
    tot_depth: f32,
    seed: i64,
    color: String,
}

impl TerrainConfiguration {
    pub fn new(tot_width: f32, tot_depth: f32, seed: i64, color: String) -> Self {
        Self {
            tot_width,
            tot_depth,
            seed,
            color,
        }
    }
}

pub fn configure_terrain(
    context: &Context,
    terrain_configuration: &TerrainConfiguration,
) -> Gm<Mesh, ColorMaterial> {
    let mut terrain: Vec<Vec<Cube>> = Vec::new();
    let z: f64 = 25.045;

    let mut width: f32 = 0.0;
    let mut depth: f32;

    while width < terrain_configuration.tot_width {
        let mut terrain_layer: Vec<Cube> = Vec::new();
        depth = 0.0;
        while depth < terrain_configuration.tot_depth {
            let value = noise3_ImproveXZ(
                terrain_configuration.seed,
                f64::from(width),
                f64::from(depth),
                z,
            );
            let dist = (width * width + depth * depth).sqrt();
            let falloff = (1.0 - (dist / 200.0)).max(0.0);
            let cube = Cube {
                x: width,
                y: depth,
                z: value * falloff + CUBE_SIZE,
            };
            terrain_layer.push(cube);
            depth += CUBE_SIZE;
        }
        terrain.push(terrain_layer);
        width += CUBE_SIZE;
    }

    let cpu_mesh = cubes_to_voxel_mesh(&terrain, terrain_configuration);

    Gm::new(
        Mesh::new(&context, &cpu_mesh),
        ColorMaterial {
            ..Default::default()
        },
    )
}

#[derive(Debug)]
struct Cube {
    x: f32,
    y: f32,
    z: f32,
}

fn add_cube(positions: &mut Vec<Vec3>, indices: &mut Vec<u32>, base: Vec3, size: f32, height: f32) {
    let start = positions.len() as u32;

    let p0 = base;
    let p1 = base + vec3(size, 0.0, 0.0);
    let p2 = base + vec3(size, height, 0.0);
    let p3 = base + vec3(0.0, height, 0.0);

    let p4 = base + vec3(0.0, 0.0, size);
    let p5 = base + vec3(size, 0.0, size);
    let p6 = base + vec3(size, height, size);
    let p7 = base + vec3(0.0, height, size);

    // 8 cube corners
    positions.extend([
        p0, p1, p2, p3, // front
        p4, p5, p6, p7, // back
    ]);

    // 12 triangles (2 per face)
    indices.extend([
        // Front
        start,
        start + 1,
        start + 2,
        start,
        start + 2,
        start + 3,
        // Back
        start + 4,
        start + 6,
        start + 5,
        start + 4,
        start + 7,
        start + 6,
        // Left
        start + 4,
        start,
        start + 3,
        start + 4,
        start + 3,
        start + 7,
        // Right
        start + 1,
        start + 5,
        start + 6,
        start + 1,
        start + 6,
        start + 2,
        // Top
        start + 3,
        start + 2,
        start + 6,
        start + 3,
        start + 6,
        start + 7,
        // Bottom
        start + 4,
        start + 5,
        start + 1,
        start + 4,
        start + 1,
        start,
    ]);
}

fn cubes_to_voxel_mesh(
    cubes: &Vec<Vec<Cube>>,
    terrain_configuration: &TerrainConfiguration,
) -> CpuMesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut colors: Vec<Srgba> = Vec::new();

    let color_r = u8::from_str_radix(&terrain_configuration.color[0..2], 16).unwrap();

    let color_g = u8::from_str_radix(&terrain_configuration.color[2..4], 16).unwrap();

    let color_b = u8::from_str_radix(&terrain_configuration.color[4..6], 16).unwrap();

    let base_color = color_g;

    for row in cubes {
        for cube in row {
            let height = cube.z.floor().max(1.0) as i32;

            // stack from ground (0) up to cube.z
            for level in 0..height {
                let base = vec3(cube.x, level as f32 * CUBE_SIZE, cube.y);

                add_cube(&mut positions, &mut indices, base, CUBE_SIZE, CUBE_SIZE);

                // darker color at bottom, lighter at top
                for _ in 0..8 {
                    let t = cube.z - ((height - level + 1) as f32 / height as f32 * 0.5);
                    let green = (base_color as f32 + 0.25 + (0.6 * t) * 50.0) as u8;
                    colors.push(Srgba::new(color_r, green, color_b, 255));
                }
            }

            if cube.z > CUBE_SIZE {
                let base = vec3(cube.x, CUBE_SIZE * height as f32, cube.y);

                add_cube(&mut positions, &mut indices, base, CUBE_SIZE, CUBE_SIZE);

                for _ in 0..8 {
                    let t = cube.z - (height as f32 / height as f32 * 0.5);
                    let green = (base_color as f32 + 0.25 + (0.6 * t) * 50.0) as u8;
                    colors.push(Srgba::new(color_r, green, color_b, 255));
                }

                let base_top = vec3(cube.x, CUBE_SIZE * height as f32 + CUBE_SIZE, cube.y);

                add_cube(
                    &mut positions,
                    &mut indices,
                    base_top,
                    CUBE_SIZE,
                    cube.z - CUBE_SIZE,
                );

                for _ in 0..8 {
                    let t = cube.z - ((height - 1) as f32 / height as f32 * 0.5);
                    let green = (base_color as f32 + 0.25 + (0.6 * t) * 50.0) as u8;
                    colors.push(Srgba::new(color_r, green, color_b, 255));
                }
            } else {
                let base = vec3(cube.x, CUBE_SIZE * height as f32, cube.y);

                add_cube(&mut positions, &mut indices, base, CUBE_SIZE, cube.z);

                for _ in 0..8 {
                    let t = cube.z - (height as f32 / height as f32 * 0.5);
                    let green = (base_color as f32 + 0.25 + (0.6 * t) * 50.0) as u8;
                    colors.push(Srgba::new(color_r, green, color_b, 255));
                }
            }
        }
    }

    CpuMesh {
        positions: Positions::F32(positions),
        indices: Indices::U32(indices),
        colors: Option::Some(colors),
        ..Default::default()
    }
}

pub fn update_configuration(
    terrain_configuration: TerrainConfiguration,
    msg: Option<ConfigurationMessage>,
) -> TerrainConfiguration {
    match msg {
        Some(ConfigurationMessage::TerrainWidth(value)) => TerrainConfiguration {
            tot_width: value,
            ..terrain_configuration
        },
        Some(ConfigurationMessage::TerrainDepth(value)) => TerrainConfiguration {
            tot_depth: value,
            ..terrain_configuration
        },
        Some(ConfigurationMessage::TerrainSeed(value)) => TerrainConfiguration {
            seed: value,
            ..terrain_configuration
        },
        Some(ConfigurationMessage::TerrainColor(value)) => TerrainConfiguration {
            color: value,
            ..terrain_configuration
        },
        None => terrain_configuration.clone(),
        _ => terrain_configuration.clone(),
    }
}
