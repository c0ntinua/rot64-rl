use raylib::prelude::*;
use raylib::core::color::Color;
use crate::calc::*;
pub fn plot(F : &[u64], screen :&mut RaylibDrawHandle) {
    let height = 32;let width = 32;
    screen.clear_background(Color {r : 0, g : 0, b: 0, a: 255});
    for r in 0..32 { for c in 0..32 {
        let col = if eval(F[r],c) == 0 {Color {r : 0, g : 0, b: 0, a: 255}} else {Color {r : 255, g : 255, b: 255, a: 255} };
        screen.draw_rectangle((c*width) as i32,(r*height)as i32,(width)as i32,(height)as i32,col);
    }}
}