use super::*;

use std::time::Instant;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use itertools::Itertools;

const GRID_SIZE_MAX: usize = 30;
const CREATE_SECONDS_MAX: usize = 10;
const NO_CHAR: char = '-';

type Grid = Vec<Vec<Cell>>;
type Offset = [isize; 2];

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
    pub char_map: BTreeMap<char, Vec<Position>>,
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
    pub position: Position,
    pub direction: Direction,
    pub intersection_count: usize,
    pub size_required: usize,
}

#[derive(Clone)]
pub struct Position {
    x: usize,
    y: usize,
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
            char_map: Default::default(),
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
        let try_count_max = 10;
        let mut try_count = 1;
        let start_time = Instant::now();
        let mut best_puzzle = None;
        let mut best_size = usize::max_value();
        loop {
            let mut puzzle = Self::new(words);
            puzzle.create();
            puzzle.print_all();
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
            try_count += 1;
            if try_count > try_count_max {
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
        let midpoint = self.get_midpoint();
        let direction = Direction::random();
        let position= match direction {
            Direction::N => Position::new(midpoint.x, self.y_max),
            Direction::NE => Position::new(self.x_min, self.y_max),
            Direction::E => Position::new(self.x_min, midpoint.y),
            Direction::SE => Position::new(self.x_min, self.y_min),
            Direction::S => Position::new(midpoint.x, self.y_min),
            Direction::SW => Position::new(self.x_max, self.y_min),
            Direction::W => Position::new(self.x_max, midpoint.y),
            Direction::NW => Position::new(self.x_max,self.y_max),
        };
        let placement = Placement::new(position, direction, 0, self.size);
        self.apply_word_placement(word, placement);
    }

    fn apply_word_placement(&mut self, word: String, placement: Placement) {
        let offset = placement.direction.get_offset();
        let mut pos = placement.position.clone();
        for char in word.chars() {
            let cell = self.get_cell_mut(&pos);
            if cell.char != NO_CHAR && cell.char != char {
                panic!("Trying to place word \"{}\" with {}. Conflicting character at {}: '{}'.",
                    &word, &placement, &pos, cell.char);
            }
            if cell.has_direction_conflict(&placement.direction) {
                panic!("Trying to place word \"{}\" with {}. Direction conflict at {}.",
                       &word, &placement, &pos);
            }
            let mut add_to_char_map = false;
            if cell.char == char {
                // placement.intersection_count += 1;
            } else {
                cell.char = char;
                add_to_char_map = true;
            }
            cell.set_direction_flag(&placement.direction);
            if add_to_char_map {
                let char_map_entry = self.char_map.entry(char).or_insert(vec![]);
                char_map_entry.push(pos.clone());
            }
            pos.apply_offset(&offset);
        }
        self.placements.insert(word, placement);
    }

    #[inline]
    fn get_midpoint(&self) -> Position {
        let x = self.x_min + ((self.x_max - self.x_min) / 2);
        let y = self.y_min + ((self.y_max - self.y_min) / 2);
        Position::new(x, y)
    }

    fn get_cell_mut(&mut self, position: &Position) -> &mut Cell {
        &mut self.grid[position.y][position.x]
    }

    pub fn get_char(&self, position: &Position) -> char {
        self.grid[position.y][position.x].char
    }

    pub fn get_char_xy(&self, x: usize, y: usize) -> char {
        self.grid[y][x].char
    }

    pub fn get_description_line(&self) -> String {
        format!("Puzzle: word count = {}; size = {}; x_min = {}, x_max = {}, y_min = {}, y_max = {}, placement count = {}, char map size = {}", self.words.len(), self.size, self.x_min, self.x_max, self.y_min, self.y_max, self.placements.len(), self.char_map.len())
    }

    pub fn print(&self, show_placements: bool, show_char_map: bool, show_puzzle: bool) {
        println!("\n{}", self.get_description_line());
        if show_placements {
            println!("\tPlacements:");
            for (word, placement) in self.placements.iter() {
                println!("\t\t\"{}\" at {}.", word, placement);
            }
        }
        if show_char_map {
            println!("\tChar Map:");
            for (char, positions) in self.char_map.iter() {
                let positions_desc = positions.iter().map(|pos| pos.to_string()).join(", ");
                println!("\t\t\"{}\" at {}.", char, positions_desc);
            }
        }
        if show_puzzle {
            self.print_puzzle();
        }
    }

    pub fn print_all(&self) {
        self.print(true, true, true);
    }

    fn print_puzzle(&self) {
        println!();
        for y in self.y_min..=self.y_max {
            let line = (self.x_min..=self.x_max).into_iter()
                .map(|x| self.get_char_xy(x, y).to_uppercase())
                .join(" ");
            println!("{}", line);
        }
        println!();
    }
}

impl Cell {
    pub fn new() -> Self {
        Self {
            char: NO_CHAR,
            has_n_s: false,
            has_ne_sw: false,
            has_e_w: false,
            has_se_nw: false
        }
    }

    pub fn has_direction_conflict(&self, direction: &Direction) -> bool {
        match direction {
            Direction::N | Direction::S => self.has_n_s,
            Direction::NE | Direction::SW => self.has_ne_sw,
            Direction::E | Direction::W => self.has_e_w,
            Direction::SE | Direction::NW => self.has_se_nw,
        }
    }

    pub fn set_direction_flag(&mut self, direction: &Direction) {
        match direction {
            Direction::N | Direction::S => self.has_n_s = true,
            Direction::NE | Direction::SW => self.has_ne_sw = true,
            Direction::E | Direction::W => self.has_e_w = true,
            Direction::SE | Direction::NW => self.has_se_nw = true,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[Cell: char = '{}'; has_n_s = {}; has_ne_sw = {}; has_e_w = {}; has_se_nw = {}]", self.char, self.has_n_s, self.has_ne_sw, self.has_e_w, self.has_se_nw)
    }
}

impl Placement {
    pub fn new(position: Position, direction: Direction, intersection_count: usize, size_required: usize) -> Self {
        Self {
            position,
            direction,
            intersection_count,
            size_required
        }
    }
}

impl Display for Placement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[Placement: position = '{}'; direction = {}; intersection_count = {}, size_required = {}]", self.position, self.direction, self.intersection_count, self.size_required)
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        assert!(x < GRID_SIZE_MAX * 2);
        assert!(y < GRID_SIZE_MAX * 2);
        Self {
            x,
            y
        }
    }

    #[inline]
    pub fn apply_offset(&mut self, offset: &Offset) {
        self.x = (self.x as isize + offset[0]) as usize;
        self.y = (self.y as isize + offset[1]) as usize;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Direction {
    pub fn get_offset(&self) -> Offset {
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

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let dir_desc = match self {
            Direction::N => "N",
            Direction::NE => "NE",
            Direction::E => "E",
            Direction::SE => "SE",
            Direction::S => "S",
            Direction::SW => "SW",
            Direction::W => "W",
            Direction::NW => "NW",
        };
        write!(f, "{}", dir_desc)
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
