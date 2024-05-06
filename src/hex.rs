use bevy::{ math::vec2, prelude::*, transform::commands};
use bevy::sprite::MaterialMesh2dBundle;
pub const HEX_OUTER_RADIUS: f32= 50.0;
pub const HEX_INNER_RADIUS: f32 = HEX_OUTER_RADIUS*0.866025404;

// fn inner_radius() ->f64{
//     return HEX_OUTER_RADIUS*0.866025404
// }

#[derive(Component)]
pub struct Hex{
    pub position : Vec3,
    pub index : Vec3,
}
impl Hex {
    fn new( x:f32, y:f32, xi:f32, yi:f32) -> Hex{
        Hex {
            position:Vec3{x:x,y:y,z:0.0},
            index:Vec3{x:xi, y:yi, z:-xi-yi},
    }}

    pub fn print_hex(&self){
    println!(
        "Position: x: {} y: {}, z: {}", self.position.x, self.position.y, self.position.z
    );
    }
}

pub fn build_hexes(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,) {
    
    // Hex Setup
    let hexes = get_hex_grid();
    for i in hexes{
        //i.print_hex();
        commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(RegularPolygon::new(1.0, 6)).into(),
        transform: Transform::default().with_scale(Vec3::splat(HEX_OUTER_RADIUS)).with_translation(i.position),
        material: materials.add(Color::PURPLE),
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
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                    format!("x:{}\n y: {}\nz: {}", i.index.x,i.index.y, i.index.z),
                    text_style.clone(),
                    ),

                ], 
                justify:text_justification,
                ..Default::default()
            
            },
            
            
            transform: Transform::default().with_translation(Vec3::new(i.position.x,i.position.y,1.0)),
            ..default()
            },
            ));
}}


fn get_hex_grid()->Vec<Hex>{
    let mut hex_vec = Vec::with_capacity(100);
    for y in 0..11 {
        for x in 0..11 {
            println!("x {} z {}", x, y);
            // hex_vec.push(Hex::new(  (x as f32 +  y as f32 * 0.5) - (y as f32 / 2.0 as f32) * (x as f32 * HEX_INNER_RADIUS * 2.0 ), 
            //                          y as f32 *HEX_OUTER_RADIUS * 1.5 as f32))
            print!("{} {}",y, y as f32* 0.5);
            hex_vec.push(Hex::new((x as f32 -5.0  + (y as f32* 0.5) - ( y/2) as f32)  * (HEX_INNER_RADIUS * 2.0) ,
                                    (y  as f32 -5.0)*HEX_OUTER_RADIUS*1.5, x as f32, y as f32))
        }
    }
    
    return hex_vec
    
}
