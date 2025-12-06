use crate::configuration::ConfigurationMessage;
use crate::smooth::noise3_ImproveXZ;
use three_d::*;

#[derive(Debug, Clone)]
pub struct TerrainConfiguration {
    tot_width: f32,
    tot_depth: f32,
    seed: i64,
    cube_size: f32,
    color: String,
    max_height: f32,
    failoff: f32,
    z: f64,
    fractal_octaves: i32,
    fractal_amplitude: f32,
    fractal_frequency: f64,
}

impl TerrainConfiguration {
    pub fn new(
        tot_width: f32,
        tot_depth: f32,
        seed: i64,
        cube_size: f32,
        color: String,
        max_height: f32,
        failoff: f32,
        z: f64,
        fractal_octaves: i32,
        fractal_amplitude: f32,
        fractal_frequency: f64,
    ) -> Self {
        Self {
            tot_width,
            tot_depth,
            seed,
            cube_size,
            color,
            max_height,
            failoff,
            z,
            fractal_octaves,
            fractal_amplitude,
            fractal_frequency,
        }
    }
}

fn fractal_noise(
    terrain_configuration: &TerrainConfiguration,
    width: f32,
    depth: f32
) -> f32 {
    let mut height: f32 = 0.0;
    let mut amplitude: f32 = 1.0;
    let mut frequency: f64 = 1.0;
    let octaves: i32 = terrain_configuration.fractal_octaves;
    for _i in 0..octaves {
        height += noise3_ImproveXZ(
            terrain_configuration.seed,
            f64::from(width) * frequency,
            f64::from(depth) * frequency,
            terrain_configuration.z,
        ) * amplitude;
        amplitude *= terrain_configuration.fractal_amplitude;
        frequency *= terrain_configuration.fractal_frequency;
    }
    let max_height = 2.0 * (1.0 - 0.5_f32.powi(octaves));

    // Normalize to [0,1]
    (height / max_height + 1.0) * 0.5
}

const CURVE: &[(f32, f32)] = &[
    (0.0, 0.0),
    (0.2, 0.02),
    (0.3, 0.05),
    (0.5, 0.15),
    (0.7, 0.35),
    (0.85, 0.6),
    (0.95, 0.8),
    (1.0, 1.0),
];

fn piecewise_linear(x: f32) -> f32 {
    for w in CURVE.windows(2) {
        let (x0, y0) = w[0];
        let (x1, y1) = w[1];

        if x <= x1 {
            return y0 + (x - x0) * (y1 - y0) / (x1 - x0);
        }
    }
    CURVE.last().unwrap().1
}

fn adjust_height(terrain_configuration: &TerrainConfiguration, height: f32) -> f32 {
    height * terrain_configuration.max_height
}

pub fn configure_terrain(
    context: &Context,
    terrain_configuration: &TerrainConfiguration,
) -> Gm<Mesh, ColorMaterial> {
    let mut terrain: Vec<Vec<Cube>> = Vec::new();

    let mut width: f32 = 0.0;
    let mut depth: f32;

    while width < terrain_configuration.tot_width {
        let mut terrain_layer: Vec<Cube> = Vec::new();
        depth = 0.0;
        while depth < terrain_configuration.tot_depth {
            let value = fractal_noise(terrain_configuration, width, depth);
            let value_piecewise = piecewise_linear(value);
            let stretch_value = adjust_height(terrain_configuration, value_piecewise);
            let dist = (width * width + depth * depth).sqrt();
            let falloff = (1.0 - (dist / terrain_configuration.failoff)).max(0.0);
            let cube = Cube {
                x: width,
                y: depth,
                z: stretch_value * falloff + terrain_configuration.cube_size,
            };
            terrain_layer.push(cube);
            depth += terrain_configuration.cube_size;
        }
        terrain.push(terrain_layer);
        width += terrain_configuration.cube_size;
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

    let base_color_red = color_r;
    let base_color_green = color_g;
    let base_color_blue = color_b;

    for row in cubes {
        for cube in row {
            let height_trunc = cube.z / terrain_configuration.cube_size;
            let fractional_part = cube.z % terrain_configuration.cube_size;

            let top_level = height_trunc.floor() as i32;

            // stack from ground (0) up to cube.z
            let mut height: f32 = 0.0;
            for level in 1..top_level {
                let base = vec3(cube.x, height, cube.y);

                add_cube(&mut positions, &mut indices, base, terrain_configuration.cube_size, terrain_configuration.cube_size);

                // darker color at bottom, lighter at top
                for _ in 0..8 {
                    let t = cube.z - ((top_level as f32 - level as f32 + 2.0) as f32 / top_level as f32 * 0.5);
                    let red = (base_color_red as f32 + 0.25 + (0.45 * t) * 50.0) as u8;
                    let green = (base_color_green as f32 + 0.25 + (0.45 * t) * 50.0) as u8;
                    let blue = (base_color_blue as f32 + 0.25 + (0.45 * t) * 50.0) as u8;
                    colors.push(Srgba::new(red, green, blue, 255));
                }
                height += terrain_configuration.cube_size;
            }

            let base = vec3(cube.x, height, cube.y);

            add_cube(
                &mut positions,
                &mut indices,
                base,
                terrain_configuration.cube_size,
                fractional_part,
            );

            for _ in 0..8 {
                let t = cube.z - (2.0 as f32 / top_level as f32 * 0.5);
                let red = (base_color_red as f32 + 0.25 + (0.45 * t) * 50.0) as u8;
                let green = (base_color_green as f32 + 0.25 + (0.45 * t) * 50.0) as u8;
                let blue = (base_color_blue as f32 + 0.25 + (0.45 * t) * 50.0) as u8;
                colors.push(Srgba::new(red, green, blue, 255));
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
        Some(ConfigurationMessage::TerrainCubeSize(value)) => TerrainConfiguration {
            cube_size: value,
            ..terrain_configuration
        },
        Some(ConfigurationMessage::TerrainColor(value)) => TerrainConfiguration {
            color: value,
            ..terrain_configuration
        },
        Some(ConfigurationMessage::TerrainMaxHeight(value)) => TerrainConfiguration {
            max_height: value,
            ..terrain_configuration
        },
        Some(ConfigurationMessage::TerrainFailoff(value)) => TerrainConfiguration {
            failoff: value,
            ..terrain_configuration
        },
        Some(ConfigurationMessage::TerrainZ(value)) => TerrainConfiguration {
            z: value,
            ..terrain_configuration
        },
        Some(ConfigurationMessage::TerrainFractalOctaves(value)) => TerrainConfiguration {
            fractal_octaves: value,
            ..terrain_configuration
        },
        Some(ConfigurationMessage::TerrainFractalAmplitude(value)) => TerrainConfiguration {
            fractal_amplitude: value,
            ..terrain_configuration
        },
        Some(ConfigurationMessage::TerrainFractalFrequency(value)) => TerrainConfiguration {
            fractal_frequency: value,
            ..terrain_configuration
        },
        None => terrain_configuration.clone(),
        _ => terrain_configuration.clone(),
    }
}
