use bevy::math::vec3;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{math::vec2, prelude::*, transform::commands};
use rand::prelude::*;
use std::fmt;

// HEX Const values
pub const HEX_OUTER_RADIUS: f32 = 50.0;
pub const HEX_INNER_RADIUS: f32 = HEX_OUTER_RADIUS * 0.866025404;

#[derive(Component, Debug, Clone, Copy)]
pub struct Hex {
    pub position: Vec3,
    pub index: Vec3,
    pub biome: Biome,
}
impl Hex {
    fn new(x: f32, y: f32, xi: f32, yi: f32, new_biome: Biome) -> Hex {
        Hex {
            position: Vec3 { x: x, y: y, z: 0.0 },
            index: Vec3 {
                x: xi,
                y: yi,
                z: -xi - yi,
            },
            biome: new_biome,
        }
    }

    pub fn check_adjacent(&self, new_index: Vec3) -> bool {
        let check: Vec<i32> = vec![
            (self.index.x - new_index.x).abs() as i32,
            (self.index.y - new_index.y).abs() as i32,
            (self.index.z - new_index.z).abs() as i32,
        ];
        if check.iter().max().unwrap() <= &2 && check.contains(&1) {
            return true;
        } else {
            return false;
        }
    }

    pub fn print_hex(&self) {
        println!(
            "Position: x: {} y: {}, z: {}\nBiome: {}",
            self.position.x, self.position.y, self.position.z, self.biome
        );
    }
}

pub fn build_hexes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Hex Setup
    let hexes = get_hex_grid();
    let hexes = hex_wave_function_collapse(hexes);

    for i in hexes {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(RegularPolygon::new(1.0, 6)).into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(HEX_OUTER_RADIUS))
                .with_translation(i.position),
            material: materials.add(i.biome.get_biome_material()),
            ..default()
        });

        // Debug Text Setup
        let font = asset_server.load("fonts\\F25_Bank_Printer.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 12.0,
            color: Color::WHITE,
        };
        let text_justification = JustifyText::Center;

        //Spawn debug text
        commands.spawn((Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!("x:{}\n y: {}\nz: {}", i.index.x, i.index.y, i.index.z),
                    text_style.clone(),
                )],
                justify: text_justification,
                ..Default::default()
            },

            transform: Transform::default().with_translation(Vec3::new(
                i.position.x,
                i.position.y,
                1.0,
            )),
            ..default()
        },));
    }
}

fn get_hex_grid() -> Vec<Hex> {
    let mut hex_vec = Vec::with_capacity(100);

    for y in 0..11 {
        for x in 0..11 {
            //println!("x {} z {}", x, y);
            //print!("{} {}", y, y as f32 * 0.5);
            hex_vec.push(Hex::new(
                (x as f32 - 5.0 + (y as f32 * 0.5) - (y / 2) as f32) * (HEX_INNER_RADIUS * 2.0),
                (y as f32 - 5.0) * HEX_OUTER_RADIUS * 1.5,
                x as f32, //index
                y as f32, //index
                Biome::Empty,
            ))
        }
    }
    return hex_vec;
}

fn hex_wave_function_collapse(mut hexes: Vec<Hex>) -> Vec<Hex> {
    // Rules:
    // All can be next to their own type
    // Grassland can be next to any but Mountain
    // Forest can be next to Grassland or Ocean
    // Ocean can be next to all but mountain
    // Sand can be next to Grassland, Ocean, Desert, or Mountain
    // Desert can be next to Mountain, Sand, Ocean, Grassland
    // Mountain can be next to Desert, Sand
    let mut rng = rand::thread_rng();
    //let biome = generate_biome(&mut rng);
    let mut new_hexes: Vec<Hex> = vec![];
    //println!("x:{}\n y: {}\nz: {}", hexes[rng.gen_range(0..hexes.len())].index.x, hexes[].index.y, hexes[rng.gen_range(0..hexes.len())].index.z, );
    for _i in 0..hexes.len() {
        let current_look = rng.gen_range(0..hexes.len());
        hexes[current_look].biome = generate_biome(&mut rng);
        let mut possible_biomes: Vec<Biome> = vec![
            Biome::Grassland,
            Biome::Forest,
            Biome::Ocean,
            Biome::Sand,
            Biome::Desert,
            Biome::Mountain,
        ];
        for hex in new_hexes.clone() {
            if hex.check_adjacent(hexes[current_look].index) {
                possible_biomes = collapse(hex.biome, possible_biomes);
            }
        }
        possible_biomes.len();
        hexes[current_look].biome = possible_biomes[rng.gen_range(0..possible_biomes.len())];

        new_hexes.push(hexes[current_look].clone());
        //hexes[current_look].print_hex();
        hexes.swap_remove(current_look);
    }

    for i in 0..new_hexes.len() {
        new_hexes[i].print_hex()
    }

    return new_hexes;
}

fn collapse (biome:Biome, mut possible_biomes:Vec<Biome>) -> Vec<Biome>{
    match biome {
        Biome::Grassland => {
            for biome_index in 0..possible_biomes.len() {
                if possible_biomes.len() > biome_index {
                    match possible_biomes[biome_index] {
                        Biome::Mountain => {
                            possible_biomes.swap_remove(biome_index);
                            ()
                        }
                        _ => (),
                    }
                }
            }
        }
        Biome::Forest => {
            for biome_index in 0..possible_biomes.len() {
                if possible_biomes.len() > biome_index {
                    match possible_biomes[biome_index] {
                        Biome::Mountain => {
                            possible_biomes.swap_remove(biome_index);
                            ()
                        }
                        Biome::Sand => {
                            possible_biomes.swap_remove(biome_index);
                            ()
                        }
                        Biome::Desert => {
                            possible_biomes.swap_remove(biome_index);
                            ()
                        }
                        _ => (),
                    }
                }
            }
        }
        Biome::Ocean => {
            for biome_index in 0..possible_biomes.len() {
                if possible_biomes.len() > biome_index {
                    match possible_biomes[biome_index] {
                        Biome::Mountain => {
                            possible_biomes.swap_remove(biome_index);
                            ()
                        }
                        _ => (),
                    }
                }
            }
        }
        Biome::Sand => {
            for biome_index in 0..possible_biomes.len() {
                if possible_biomes.len() > biome_index {
                    match possible_biomes[biome_index] {
                        Biome::Forest => {
                            possible_biomes.swap_remove(biome_index);
                            ()
                        }
                        _ => (),
                    }
                }
            }
        }
        Biome::Desert => {
            for biome_index in 0..possible_biomes.len() {
                if possible_biomes.len() > biome_index {
                    match possible_biomes[biome_index] {
                        Biome::Forest => {
                            possible_biomes.swap_remove(biome_index);
                            ()
                        }
                        _ => (),
                    }
                }
            }
        }
        Biome::Mountain => {
            for biome_index in 0..possible_biomes.len() {
                if possible_biomes.len() > biome_index {
                    match possible_biomes[biome_index] {
                        Biome::Forest => {
                            possible_biomes.swap_remove(biome_index);
                            ()
                        }
                        Biome::Ocean => {
                            possible_biomes.swap_remove(biome_index);
                            ()
                        }
                        Biome::Forest => {
                            possible_biomes.swap_remove(biome_index);
                            ()
                        }
                        _ => (),
                    }
                }
            }
        }
        _ => {}
    }
    return possible_biomes
}

#[derive(Debug, Clone, Copy)]
pub enum Biome {
    Grassland,
    Forest,
    Ocean,
    Sand,
    Desert,
    Mountain,
    Empty,
}

fn generate_biome<R: Rng + ?Sized>(rng: &mut R) -> Biome {
    match rng.gen_range(0..6) {
        // rand 0.8
        0 => Biome::Grassland,
        1 => Biome::Forest,
        2 => Biome::Ocean,
        3 => Biome::Sand,
        4 => Biome::Desert,
        5 => Biome::Mountain,
        _ => Biome::Empty,
    }
}

impl Biome {
    fn get_biome_material(&self) -> Color {
        match self {
            Biome::Grassland => Color::GREEN,
            Biome::Forest => Color::DARK_GREEN,
            Biome::Ocean => Color::MIDNIGHT_BLUE,
            Biome::Sand => Color::BEIGE,
            Biome::Desert => Color::YELLOW,
            Biome::Mountain => Color::SILVER,
            Biome::Empty => Color::PURPLE,
        }
    }
}

impl fmt::Display for Biome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Biome::Grassland => write!(f, "Grassland"),
            Biome::Forest => write!(f, "Forest"),
            Biome::Ocean => write!(f, "Ocean"),
            Biome::Sand => write!(f, "Sand"),
            Biome::Desert => write!(f, "Desert"),
            Biome::Mountain => write!(f, "Mountain"),
            Biome::Empty => write!(f, "Empty"),
        }
    }
}
