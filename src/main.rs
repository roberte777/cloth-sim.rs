use cloth_sim::cloth::Cloth;
use macroquad::prelude::*;
const WIDTH: usize = 19;
const HEIGHT: usize = 11;
const SPACING: f32 = 15.0;

#[macroquad::main("BasicShapes")]
async fn main() {
    clear_background(BLACK);
    let mut cloth = Cloth::new(WIDTH, HEIGHT, SPACING);
    cloth.draw();
    loop {
        clear_background(BLACK);
        check_mouse(&mut cloth);
        cloth.update();
        cloth.draw();
        next_frame().await
    }
}

fn check_mouse(cloth: &mut Cloth) {
    let mouse_pos = mouse_position();
    //cut a selected stick
    if is_mouse_button_pressed(MouseButton::Left) {
        cloth.cut_stick(mouse_pos.into());
    }
}
