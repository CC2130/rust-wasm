mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

use rand::prelude::*;

extern crate web_sys;
use web_sys::console;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer {
            name,
        }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

/// 生命游戏：
/// 由一个二维网格所表示的无限宇宙，每个网格表示一个生命，生命的状态遵循下面四个规则：
/// 1. 任何四周邻居存活数少于两个的存活网格将死亡
/// 2. 任何四周邻居存活数为两个或三个的存活网格将在下一代继续存活
/// 3. 任何四周邻居存活数多于三个的存活网格将死亡
/// 4. 任何已经死亡的网格，如果周围邻居存活数为三个，将在下一代复活
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}


/// 比如，一个三行三列的 Universe, 
/// [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ]
/// |  row0  |  row1  |  row2  |
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = vec![];
        let mut universe = Universe {
            width,
            height,
            cells,
        };

        // 随机生成 Cell 状态
        universe.start();

        universe
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    /// 调用进行所有生命的状态更新
    pub fn tick(&mut self) {
        //let _time = Timer::new("Universe::tick");
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, column);

                //let state = cell;

                let next_cell = match(cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                // console.log
                //if next_cell != state {
                //    log!("the {} {} cell have transited from {:?} to {:?}", row, column, state, next_cell);
                //}

                next[index] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let index = self.get_index(row, column);
        self.cells[index].toggle();
    }

    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = Cell::Dead;
        }
        log!("Reset all Cells to Dead!");
    }

    pub fn start(&mut self) {
        // 随机生成 Cell 状态
        let cells = (0..self.width * self.height)
            .map(|_| {
                if random() {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        self.cells = cells;
    }
}

impl Universe {
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        //  上下左右四个方位
        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let ws = self.get_index(south, west);
        count += self.cells[ws] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        count
    }

    /// 通过 row, column 获得在 self.cells 中的位置 id
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// 获取 self.cells 
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// 将数组中的 Cell 设置为存活状态
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, column) in cells.iter().cloned() {
            let index = self.get_index(row, column);
            self.cells[index] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
