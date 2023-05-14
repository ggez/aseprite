extern crate aseprite;
extern crate serde_json;

use std::fs::File;

fn main() {
    let file = File::open("boonga.json").unwrap();
    let spritesheet: aseprite::SpritesheetData = serde_json::from_reader(file).unwrap();
    println!("Spritesheet is {:?}", spritesheet);
}
