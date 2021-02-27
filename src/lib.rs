mod game_state;
mod utils;
mod gl_setup;

use rand::Rng;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

// Use `wee_alloc` as the global allocator when this feature is enabled
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Use Console.log throughout the Rust code to aid debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Keep this in for now just to confirm the web page is still loading WASM properly
#[wasm_bindgen]
pub fn verify() {
    log("WASM loaded");
}

// Inspired by https://rustwasm.github.io/docs/book/game-of-life/implementing.html
// Define universe as a Rectangular grid of cells (unsigned integers) in the WASM linear memory space
// Expose the universe to JavaScript as a 1D array using `index(row, column, universe) = row * width(universe) + column`
// JavaScript is then responsible for looping through this and re-rendering (may come back to this, aiming for ~0 JavaScript)

#[wasm_bindgen]
#[repr(u8)] // each cell represented as a single byte, allows 255 possible states for a cell, should be sufficient for a simple game
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Block = 1,
    Player1 = 2,
    Player2 = 3, 
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}
// Create to_string method for our universe - convert u8 to display character and split into rows
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Player1 => '>',
                    Cell::Player2 => '<',
                    Cell::Block => '◼',
                    _ => '◻'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    // Construct universe based on rules inferred from examining https://archive.org/details/MiceMen_1020 (lost the original CD)
    // Columns 1, 21               == transpose ◻◻◻◻◻◻◻◻◻◻◻◻◻
    // Column 11                   == transpose ◻◼◼◻◼◼◻◼◼◻◼◼◻
    // Columns 2,6,10,12,16,20     == transpose ◼◻◻◼◻◻◼◻◻◼◻◻◼
    // Columns 3-5,7-9,13-15,17-19 == randomly scatter 7 blocks (original game varies 5-8, rarely 5, average just under 22, in each set of 3 blocks)
    // 12 total of each player seem to be scattered across remaining empty cells, player 1 on left, player 2 on right
    // I want more consistency than this - 1 mouse per column + 1 extra mouse in the special columns with only 5 blocks
    pub fn new() -> Universe {
        let width = 21;
        let height = 13;
        
        let mut cells = Vec::new();
        
        for row in 1..height+1 {
            for col in 1..width+1 {
                cells.push(
                    match col {
                        1 | 21  => Cell::Empty,
                        11      =>  match row {
                                        2|3|5|6|8|9|11|12   => Cell::Block,
                                        _                   => Cell::Empty,
                                    },
                        2|6|10 =>   match row {
                                        1|4|7|10|13         => Cell::Block,
                                        3|6|9|12            => Cell::Player1,// TODO: more random, yet predictable spread
                                        _                   => Cell::Empty,
                                    },
                        12|16|20 => match row {
                                        1|4|7|10|13         => Cell::Block,
                                        3|6|9|12            => Cell::Player2,// TODO: more random, yet predictable spread
                                        _                   => Cell::Empty,
                                    },
                        _ => if rand::thread_rng().gen_range(1..=13) <= 7 { Cell::Block }else{Cell::Empty},
                        // TODO: less random blocks; 6-8 blocks per col, equal split L/R, no groups of more than 4 blocks in a column
                    });
            }
        }

        Universe {
            width,
            height,
            cells,
        }
    }


    pub fn update(&mut self, 
        //_player: game_state::Player, 
        //_column: game_state::Column, 
        //_action: game_state::Action
    ) {

        
        
        let mut _next = self.cells().clone();

    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    // fn get_index(&self, row: u32, column: u32) -> usize {
    //     (row * self.width + column) as usize
    // }
}




//
//
// WebGL stuff follows but I need to spend more time understanding the Rust/Wasm/JS boundary first without the overhead of learning about Graphics/Shaders
//
//
#[wasm_bindgen]
pub struct AppClient {
    gl: WebGlRenderingContext, // Graphics layer
}
#[wasm_bindgen]
impl AppClient {
 
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        log("AppClient Constructor called");
        let gl = gl_setup::initialize_webgl_context().unwrap();
        Self {
            gl: gl,
        }
    }
    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        log("AppClient.update called");
        Ok(())
    }
    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        log("AppClient.render called");
    }
}