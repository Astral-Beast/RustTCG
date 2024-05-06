use std::{vec, f64};
use bevy::prelude::*;

const HEX_OUTER_RADIUS: f64 = 50.0;
const HEX_INNER_RADIUS: f64 = HEX_OUTER_RADIUS*0.866025404;

struct HexPosition{
    x:i16,
    y: i16,
    z:i16,
}

// fn inner_radius() ->f64{
//     return HEX_OUTER_RADIUS*0.866025404
// }

#[derive(Component)]
pub struct Hex{
    position : HexPosition,
}
impl Hex {
    fn new( x:i16, z:i16) -> Hex{
        Hex {position:HexPosition{x:x,y:0,z:z}}        
    }
    pub fn print_hex(&self){
    println!(
        "Position: x: {} y: {}, z: {}", self.position.x, self.position.y, self.position.z
    );
    }
}

pub fn build_hexes() -> Vec<Hex>{
    let (width, height, scale) = (1024, 1024, 128.0);
    let mut hex_vec = Vec::with_capacity(100);
    for x in 1..10 {
        for z in 1..10 {
            hex_vec.push(Hex::new(x,z))
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

