use super::*;

use std::time::Instant;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::collections::BTreeMap;

const GRID_SIZE_MAX: usize = 30;
const CREATE_SECONDS_MAX: usize = 10;

type Grid = Vec<Vec<Cell>>;

#[derive(Clone)]
pub struct Puzzle {
    pub words: Vec<String>,
    pub grid: Grid,
    pub size: usize,
    pub x_min: usize,
    pub x_max: usize,
    pub y_min: usize,
    pub y_max: usize,
    pub placements: BTreeMap<String, Placement>,
}

#[derive(Clone)]
pub struct Cell {
    char: char,
    has_n_s: bool,
    has_ne_sw: bool,
    has_e_w: bool,
    has_se_nw: bool,
}

#[derive(Clone)]
pub struct Placement {
    pub position: (usize, usize),
    pub direction: Direction,
    pub intersection_count: usize,
    pub size_required: usize,
}

#[derive(Clone)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Puzzle {
    pub fn new(words: &Vec<String>) -> Self {
        debug_assert!(!words.is_empty());
        let words = words.iter().map(|word| word.trim().to_lowercase()).collect::<Vec<_>>();
        let size = words.iter().map(|word| word.len()).max().unwrap();
        debug_assert!(size <= GRID_SIZE_MAX);
        let x_max = GRID_SIZE_MAX + (size / 2);
        let x_min = x_max - (size - 1);
        let y_max = x_max;
        let y_min = x_min;
        Self {
            words,
            grid: Self::create_grid(),
            size,
            x_min,
            x_max,
            y_min,
            y_max,
            placements: Default::default(),
        }
    }

    fn create_grid() -> Grid {
        let grid_size = GRID_SIZE_MAX * 2;
        let mut grid = Vec::with_capacity(grid_size);
        for _y in 0..grid_size {
            let mut row = Vec::with_capacity(grid_size);
            for _x in 0..grid_size {
                row.push(Cell::new());
            }
            grid.push(row);
        }
        grid
    }

    pub fn find_best_puzzle(words: &Vec<String>) -> Option<Self> {
        let start_time = Instant::now();
        let mut best_puzzle = None;
        let mut best_size = usize::max_value();
        loop {
            let mut puzzle = Self::new(words);
            puzzle.create();
            let puzzle_size = puzzle.size;
            if puzzle_size < best_size {
            // let new_is_better = best_puzzle.map_or(true, |best_puzzle: Puzzle| best_puzzle.size > puzzle.size);
            // let new_is_better = true;
            // if new_is_better {
                best_puzzle = Some(puzzle);
                best_size = puzzle_size;
            }
            let elapsed = (Instant::now() - start_time).as_secs();
            if elapsed > CREATE_SECONDS_MAX as u64 {
                break;
            }
        }
        best_puzzle
    }

    fn create(&mut self) {
        self.words.shuffle(&mut thread_rng());
        self.words.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
        //bg!(&self.words);
        self.place_first_word();



    }

    fn place_first_word(&mut self) {
        let word = self.words.remove(0);
        let direction = Direction::random();
        let x_y = match direction {
            Direction::N => (self.)
            Direction::NE => {}
            Direction::E => {}
            Direction::SE => {}
            Direction::S => {}
            Direction::SW => {}
            Direction::W => {}
            Direction::NW => {}
        }
    }

}

impl Cell {
    pub fn new() -> Self {
        Self {
            char: ' ',
            has_n_s: false,
            has_ne_sw: false,
            has_e_w: false,
            has_se_nw: false
        }
    }
}

impl Placement {
    pub fn new(position: (usize, usize), direction: &Direction, intersection_count: usize, size_required: usize) -> Self {
        Self {
            position,
            direction: direction.clone(),
            intersection_count,
            size_required
        }
    }

}

impl Direction {
    pub fn get_offset(&self) -> [isize; 2] {
        match self {
            Direction::N => [0, -1],
            Direction::NE => [1, -1],
            Direction::E => [1, 0],
            Direction::SE => [1, 1],
            Direction::S => [0, 1],
            Direction::SW => [-1, 1],
            Direction::W => [-1, 0],
            Direction::NW => [-1, -1],
        }
    }

    pub fn random() -> Self {
        let index = thread_rng().gen_range(0..8);
        match index {
            0 => Direction::N,
            1 => Direction::NE,
            2 => Direction::E,
            3 => Direction::SE,
            4 => Direction::S,
            5 => Direction::SW,
            6 => Direction::W,
            7 => Direction::NW,
            _ => unreachable!(),
        }
    }

}

pub fn slice_str_to_strings(list: &[&str]) -> Vec<String> {
    list.iter().map(|x| x.to_string()).collect()
}

pub fn vec_str_to_strings(list: &Vec<&str>) -> Vec<String> {
    list.iter().map(|x| x.to_string()).collect()
}

pub fn main() {
    Puzzle::find_best_puzzle(&slice_str_to_strings(&word_list::WORDS_1.to_vec()));
}
