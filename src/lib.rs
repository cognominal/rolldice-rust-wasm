use js_sys::Math;
use web_sys;
use wasm_bindgen::prelude::*;
import * as wasm from './my_wasm_project_bg';


// Roll a number of dice with a certain number of sides each
#[wasm_bindgen(js_name = "roll_dice")]
pub fn roll_dice(num_dice: u32, num_sides: u32) -> u32 {
    let mut total = 0;
    for _ in 0..num_dice {
        let roll = Math::floor(Math::random() * num_sides as f64) as u32 + 1;
        total += roll;
    }
    total
}

// Add the result of rolling dice to the DOM
pub fn add_roll_to_dom(num_dice: u32, num_sides: u32) {
    let result = roll_dice(num_dice, num_sides);
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let p = document.create_element("p").unwrap();
    p.set_inner_html(&format!("You rolled {} dice with {} sides each and got {}", num_dice, num_sides, result));
    let body = document.body().unwrap();
    body.append_child(&p).unwrap();
}