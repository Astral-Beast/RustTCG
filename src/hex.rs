use bevy::{ math::vec2, prelude::*};

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

pub fn build_hexes() -> Vec<Hex>{
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


// impl fmt::Display for Hex {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match &self {
//             HexPosition => write!(f, "Position: x: {} y: {}, z: {}", x, y, z),
//         }
//     }
// }

