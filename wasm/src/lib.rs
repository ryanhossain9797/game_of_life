use std::collections::HashSet;

use game_of_life::{GameState, Generation};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Life {
    inner: GameState,
}

#[wasm_bindgen]
pub struct WasmPoint(u32, u32);

#[wasm_bindgen]
impl Life {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: Vec<WasmPoint>, width: u32, height: u32) -> Life {
        let generation = Generation {
            live_cells: initial
                .into_iter()
                .map(|p| game_of_life::Point::new(p.0 as usize, p.1 as usize))
                .collect::<HashSet<_>>(),
            x_max: width as usize,
            y_max: height as usize,
        };

        Life {
            inner: GameState::new(generation),
        }
    }

    pub fn tick(&mut self) -> Vec<u32> {
        let generation = self.inner.next().unwrap();

        let mut out = Vec::with_capacity(generation.live_cells.len() * 2);

        for p in &generation.live_cells {
            out.push(p.x as u32);
            out.push(p.y as u32);
        }

        out
    }
}
