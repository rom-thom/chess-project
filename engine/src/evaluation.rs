// For static evaluation
use chess_core::position::{Color, Position};
use chess_core::piece::PieceIndex;
use chess_core::board::Bitboards;


const VALS: [i32; 5] = [100, 320, 330, 500, 900]; // P,N,B,R,Q

// Low-level, pure, easy to unit-test
#[inline(always)]
fn material_from_bitboards(bb: &Bitboards, color: Color) -> i32 {
    let pcs = bb.color_slice(color);           // [P,N,B,R,Q,K]
    let mut sum = 0i32;
    for (k, &b) in pcs.iter().take(5).enumerate() {
        sum += (b.count_ones() as i32) * VALS[k]; // VALS = [100,320,330,500,900]
    }
    if matches!(color, Color::White) { sum } else { -sum }
}

// Public API – flexible to evolve later
#[inline]
pub fn evaluate_material(pos: &Position, color: Color) -> i32 {
    // If you add a material cache, return it here:
    // return pos.material_cache[color as usize];

    material_from_bitboards(&pos.current.bitboards, color)
}





pub fn evaluate(pos: &Position) -> i32 { 
    todo!("this is an evaluation of the static position")
 }



 #[test]
 fn test_eval(){
    let pos = Position::new(None);
    dbg!(evaluate_material(&pos, Color::White));
 }