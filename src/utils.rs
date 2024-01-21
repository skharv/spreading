pub const COLOUR_PLAYER: [u8; 4] = [255, 194, 122, 255];
pub const COLOUR_DARK_FLOOR: [u8; 4] = [53, 93, 104, 255];
pub const COLOUR_LIGHT_FLOOR: [u8; 4] = [66, 116, 120, 255];
//pub const COLOUR_BUBBLE: [u8; 4] = [106, 175, 157, 255];
pub const COLOUR_BEAM: [u8; 4] = [167, 49, 105, 255];
pub const COLOUR_DEEP: [u8; 4] = [32, 17, 39, 255];
pub const COLOUR_MEDIUM: [u8; 4] = [32, 20, 51, 255];
pub const COLOUR_SHALLOW: [u8; 4] = [27, 30, 52, 255];
pub const COLOUR_RESOURCE: [u8; 4] = [236, 154, 109, 255];
pub const COLOUR_RICH_RESOURCE: [u8; 4] = [217, 98, 107, 255];
pub const COLOUR_SPREAD: [u8; 4] = [148, 197, 172, 255];
pub const COLOUR_UI: [u8; 4] = [255, 235, 153, 255];

pub const SYMBOL_RESOURCE: &str = "08181C3C3466663C";
pub const SYMBOL_HOUSE: &str = "182442FF425A5A7E";
pub const SYMBOL_BOMB: &str = "01091670F8F8F870";
pub const SYMBOL_FACE: &str = "7E81A5A58199817E";
pub const SYMBOL_FACTORY: &str = "020A2A2A7E7E7EFF";
pub const SYMBOL_BUBBLE: &str = "3C4289858581423C";
pub const SYMBOL_BASE_SHAPE: &str = "E7818100008181E7";

pub const AUTOMATON_SEEK_RANGE: f32 = 20.0;

pub enum Action {
    House,
    Bomb,
    Face,
    Factory,
    Bubble
}


pub fn convert_string_to_symbol(
    input: &String
    ) -> [bool; crate::SYMBOL_SIZE * crate::SYMBOL_SIZE] {
    let mut output = [false; crate::SYMBOL_SIZE * crate::SYMBOL_SIZE]; 
    for i in 0..input.len() {
        if let Some(input_char) = input.chars().nth(i) {
            let bools = binary_lookup(input_char);
            for j in 0..bools.len() {
                output[i*4 + j] = bools[j];
            }
        }
    }
    return output;
}

pub fn is_position_part_of_symbol(
    x: i32,
    y: i32,
    symbol: [bool; crate::SYMBOL_SIZE * crate::SYMBOL_SIZE]
    ) -> bool {
    for i in 0..symbol.len() {
        if symbol[i] {
            let symbol_x = (i % crate::SYMBOL_SIZE) as i32;
            let symbol_y = (i / crate::SYMBOL_SIZE) as i32;
            
            if x == symbol_x && y == symbol_y {
                println!("FOUND SYMBOL AT {}, {}", symbol_x, symbol_y);
                return true;
            }
        }
    }
    return false;
}

pub fn binary_lookup(
    input: char
    ) -> [bool; 4] {
    match input {
        '0' => [false, false, false, false],
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'A' => [true, false, true, false],
        'B' => [true, false, true, true],
        'C' => [true, true, false, false],
        'D' => [true, true, false, true],
        'E' => [true, true, true, false],
        'F' => [true, true, true, true],
        _ => [false, false, false, false],
    }
}


