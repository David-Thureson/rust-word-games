use super::*;

use std::time::Instant;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use itertools::Itertools;

const PUZZLE_SIZE_MAX: usize = 30;
const FIELD_SIZE_MULT: usize = 2;
const CREATE_SECONDS_MAX: usize = 10;
const NO_CHAR: char = '-';
const DIRECTIONS: [Direction; 8] = [Direction::N, Direction::NE, Direction::E, Direction::SE, Direction::S, Direction::SW, Direction::W, Direction::NW];

type Grid = Vec<Vec<Cell>>;
type Offset = [isize; 2];

#[derive(Clone)]
pub struct Puzzle {
    words: Vec<String>,
    expansion: f32,
    directions: Vec<Direction>,
    grid: Grid,
    bounds: Bounds,
    placements: BTreeMap<String, Placement>,
    char_map: BTreeMap<char, Vec<Position>>,
}

#[derive(Clone, Debug)]
pub struct Cell {
    char: char,
}

#[derive(Clone, Debug)]
pub struct Placement {
    position: Position,
    direction: Direction,
    intersection_count: usize,
    adjacent_count: usize,
    size_rank: usize,
    adjacent_rank: usize,
    bounds: Bounds,
}

#[derive(Clone, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
pub struct Bounds {
    top_left: Position,
    bottom_right: Position,
}

#[derive(Clone, Debug)]
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
    pub fn new(words: &Vec<String>, expansion: f32) -> Self {
        debug_assert!(!words.is_empty());
        let words = words.iter().map(|word| word.trim().to_lowercase()).collect::<Vec<_>>();
        let size = words.iter().map(|word| word.len()).max().unwrap();
        debug_assert!(size <= PUZZLE_SIZE_MAX);
        let x_max = PUZZLE_SIZE_MAX + (size / 2);
        let x_min = x_max - (size - 1);
        let y_max = x_max;
        let y_min = x_min;
        Self {
            words,
            expansion,
            directions: DIRECTIONS.iter().map(|x| x.clone()).collect(),
            grid: Self::create_grid(),
            bounds: Bounds::new(Position::new(x_min, y_min), Position::new(x_max, y_max)),
            placements: Default::default(),
            char_map: Default::default(),
        }
    }

    fn create_grid() -> Grid {
        let grid_size = Self::get_field_size();
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

    pub fn find_best_puzzle(words: &Vec<String>, expansion: f32, directions: Option<Vec<Direction>>) -> Option<Self> {
        let try_count_max = 10;
        let mut try_count = 1;
        let start_time = Instant::now();
        let mut best_puzzle = None;
        let mut best_size = usize::max_value();
        let mut sizes = vec![];
        loop {
            let mut puzzle = Self::new(words, expansion);
            if let Some(ref directions) = directions {
                puzzle.directions = directions.clone();
            }
            puzzle.create();
            puzzle.print_puzzle();
            let puzzle_size = puzzle.bounds.get_size();
            sizes.push(puzzle_size);
            if puzzle_size < best_size {
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
        sizes.sort();
        dbg!(sizes);
        best_puzzle
    }

    fn create(&mut self) {
        self.words.shuffle(&mut thread_rng());
        self.words.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
        let mut words = self.words.clone();
        while !words.is_empty() {
            self.place_word(words.remove(0));
        }
    }

    fn place_word(&mut self, word: String) {
        // Try all possible placements.
        let mut placements = vec![];
        for x in self.bounds.get_x_min()..=self.bounds.get_x_max() {
            for y in self.bounds.get_y_min()..=self.bounds.get_y_max() {
                let position = Position::new(x, y);
                for direction in self.directions.iter() {
                    if let Some(placement) = self.try_placement(&word, 0, &position, direction) {
                        placements.push(placement);
                    }
                }
            }
        }
        let mut chosen_placement_index = 0;
        if placements.len() > 1 {
            // Set the size rankings. The smallest sizes go first and get the smallest rank
            // numbers.
            placements.shuffle(&mut thread_rng());
            placements.sort_by(|a, b| a.bounds.get_size().cmp(&b.bounds.get_size()));
            placements.iter_mut().enumerate().for_each(|(i, placement)| placement.size_rank = i);

            // Set the adjacent count rankings. The _highest_ adjacent counts go first and get
            // the smallest rank numbers.
            placements.shuffle(&mut thread_rng());
            placements.sort_by(|a, b| a.adjacent_count.cmp(&b.adjacent_count).reverse());
            placements.iter_mut().enumerate().for_each(|(i, placement)| placement.adjacent_rank = i);

            // Sort by the combined ranks.
            placements.shuffle(&mut thread_rng());
            placements.sort_by(|a, b| (a.size_rank + a.adjacent_rank).cmp(&(b.size_rank + &b.adjacent_rank)));

            // Choose an entry from the top of the list (self.expansion is 0.0, as compact as
            // passible), the end of the list (self.expansion is 1.0, as loose as possible), or
            // somewhere in between.
            chosen_placement_index = ((placements.len() as f32 - 1.0) * self.expansion).floor() as usize;
        }
        self.apply_word_placement(word, placements.remove(chosen_placement_index));
    }
    
    fn try_placement(&self, word: &String, char_index: usize, position: &Position, direction: &Direction) -> Option<Placement> {
        let mut intersection_count = 0;
        let mut adjacent_count = 0;
        let mut bounds = self.bounds.clone();
        let position_new_word = position.back_to_word_start_optional(char_index, direction, Self::get_field_size());
        if position_new_word.is_none() {
            return None;
        }
        let position_new_word = position_new_word.unwrap();
        let mut pos = position_new_word.clone();
        if !self.is_placement_on_grid(word.len(), &pos, direction) {
            return None;
        }
        let offset = direction.get_offset();
        for char in word.chars() {
            let cell = self.get_cell(&pos);
            if cell.char != NO_CHAR && cell.char != char {
                // There's already a character in this cell and it doesn't match the character in
                // the new word.
                return None;
            }
            if cell.char == char {
                intersection_count += 1;
            } else {
                // If we use this placement we'll be adding a character at this position. Count the
                // number of filled cells touching this one.
                adjacent_count += self.get_adjacent_count(&pos, direction);
            }
            bounds.apply_position(&pos);
            pos.apply_offset(&offset);
        }
        let placement = Placement::new(position_new_word, direction.clone(), intersection_count, adjacent_count, bounds);
        Some(placement)
    }

    fn is_placement_on_grid(&self, word_length: usize, position: &Position, direction: &Direction) -> bool {
        let word_length = word_length as isize;
        let field_size = Self::get_field_size();
        let offset = direction.get_offset();
        let x_end = position.x as isize + (word_length * offset[0]);
        let y_end = position.y as isize + (word_length * offset[1]);
        position.x < field_size && x_end >= 0 && x_end < field_size as isize
            && position.y < field_size && y_end >= 0 && y_end < field_size as isize
    }

    fn get_adjacent_count(&self, position: &Position, direction: &Direction) -> usize {
        let dir_this = direction.get_variant_name().to_string();
        let dir_opposite = direction.opposite().get_variant_name().to_string();
        let mut count = 0;
        for dir in DIRECTIONS.iter()
                .filter(|dir| dir.get_variant_name().ne(dir_this.as_str()) && dir.get_variant_name().ne(dir_opposite.as_str())) {
            let offset = dir.get_offset();
            let mut pos = position.clone();
            pos.apply_offset(&offset);
            if self.get_cell(&pos).char != NO_CHAR {
                count += 1;
            }
        }
        count
    }

    fn apply_word_placement(&mut self, word: String, placement: Placement) {
        let offset = placement.direction.get_offset();
        let mut pos = placement.position.clone();
        for char in word.chars() {
            self.bounds.apply_position(&pos);
            let cell = self.get_cell_mut(&pos);
            let found_char = cell.char;
            if found_char != NO_CHAR && found_char != char {
                self.print_all();
                panic!("Trying to place word \"{}\" with {}. Conflicting character at {}: '{}'.",
                    &word, &placement, &pos, found_char);
            }
            let mut add_to_char_map = false;
            if cell.char != char {
                cell.char = char;
                add_to_char_map = true;
            }
            if add_to_char_map {
                let char_map_entry = self.char_map.entry(char).or_insert(vec![]);
                char_map_entry.push(pos.clone());
            }
            pos.apply_offset(&offset);
        }
        self.placements.insert(word, placement);
    }

    fn get_cell(&self, position: &Position) -> &Cell {
        &self.grid[position.y][position.x]
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

    pub fn get_field_size() -> usize {
        PUZZLE_SIZE_MAX * FIELD_SIZE_MULT
    }

    pub fn get_description_line(&self) -> String {
        format!("Puzzle: word count = {}; size = {}; {}; placement count = {}; char map size = {}", self.words.len(), self.bounds.get_size(), &self.bounds, self.placements.len(), self.char_map.len())
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

    pub fn print_puzzle(&self) {
        println!();
        for y in self.bounds.get_y_min()..=self.bounds.get_y_max() {
            let line = (self.bounds.get_x_min()..=self.bounds.get_x_max()).into_iter()
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
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[Cell: char = '{}']", self.char)
    }
}

impl Placement {
    fn new(position: Position, direction: Direction, intersection_count: usize, adjacent_count: usize, bounds: Bounds) -> Self {
        Self {
            position,
            direction,
            intersection_count,
            adjacent_count,
            size_rank: 0,
            adjacent_rank: 0,
            bounds,
        }
    }
}

impl Display for Placement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[Placement: position = '{}'; direction = {}; intersection_count = {}; size_rank = {}; adjacent_rank = {}; {}]",
               self.position, self.direction, self.intersection_count,
               util::format::format_count(self.size_rank),
               util::format::format_count(self.adjacent_rank), self.bounds)
    }
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        assert!(x < PUZZLE_SIZE_MAX * FIELD_SIZE_MULT);
        assert!(y < PUZZLE_SIZE_MAX * FIELD_SIZE_MULT);
        Self {
            x,
            y
        }
    }

    #[inline]
    fn apply_offset(&mut self, offset: &Offset) {
        self.x = (self.x as isize + offset[0]) as usize;
        self.y = (self.y as isize + offset[1]) as usize;
    }

    fn back_to_word_start_optional(&self, char_index: usize, direction: &Direction, field_size: usize) -> Option<Self> {
        // We're starting at the proposed intersection of a new word with an existing word at some
        // character in the new word. We need to return the position of the first character of the
        // new word.
        let char_index = char_index as isize;
        let field_size = field_size as isize;
        let offset = direction.get_offset();
        let x = self.x as isize - (char_index * offset[0]);
        let y = self.y as isize - (char_index * offset[1]);
        if x > 0 && x < field_size && y > 0 && y < field_size {
            Some(Self::new(x as usize, y as usize))
        } else {
            None
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Bounds {
    fn new(top_left: Position, bottom_right: Position) -> Self {
        let bounds = Self {
            top_left,
            bottom_right
        };
        bounds.invariant();
        bounds
    }

    #[inline]
    fn get_size(&self) -> usize {
        let x_size = (self.get_x_max() - self.get_x_min()) + 1;
        let y_size = (self.get_y_max() - self.get_y_min()) + 1;
        x_size.max(y_size)
    }

    #[inline]
    fn get_x_min(&self) -> usize {
        self.top_left.x
    }

    #[inline]
    fn get_x_max(&self) -> usize {
        self.bottom_right.x
    }

    #[inline]
    fn get_y_min(&self) -> usize {
        self.top_left.y
    }

    #[inline]
    fn get_y_max(&self) -> usize {
        self.bottom_right.y
    }

    #[inline]
    fn apply_position(&mut self, position: &Position) {
        self.top_left.x = self.top_left.x.min(position.x);
        self.bottom_right.x = self.bottom_right.x.max(position.x);
        self.top_left.y = self.top_left.y.min(position.y);
        self.bottom_right.y = self.bottom_right.y.max(position.y);
    }

    fn invariant(&self) {
        assert!(self.get_x_min() <= self.get_x_max(), "In {}, x_min ({}) > x_max ({}).", self, self.get_x_min(), self.get_x_max());
        assert!(self.get_y_min() <= self.get_y_max(), "In {}, y_min ({}) > y_max ({}).", self, self.get_y_min(), self.get_y_max());
    }
}

impl Display for Bounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[Bounds: {}-{}; size = {}]", self.top_left, self.bottom_right, self.get_size())
    }
}

impl Direction {
    fn get_offset(&self) -> Offset {
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

    fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::NE => Direction::SW,
            Direction::E => Direction::W,
            Direction::SE => Direction::NW,
            Direction::S => Direction::N,
            Direction::SW => Direction::NE,
            Direction::W => Direction::E,
            Direction::NW => Direction::SE,
        }
    }

    fn get_variant_name(&self) -> &str {
        match self {
            Direction::N => "N",
            Direction::NE => "NE",
            Direction::E => "E",
            Direction::SE => "SE",
            Direction::S => "S",
            Direction::SW => "SW",
            Direction::W => "W",
            Direction::NW => "NW",
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.get_variant_name() == other.get_variant_name()
    }
}

impl Eq for Direction {}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.get_variant_name())
    }
}

pub fn slice_str_to_strings(list: &[&str]) -> Vec<String> {
    list.iter().map(|x| x.to_string()).collect()
}

pub fn vec_str_to_strings(list: &Vec<&str>) -> Vec<String> {
    list.iter().map(|x| x.to_string()).collect()
}

pub fn main() {
    let words = word_list::WORDS_1;
    // let words = word_list::ALL_SECOND_GRADE;
    let expansion = 0.2;
    // let directions = None;
    // let directions = Some(vec![Direction::E, Direction::S]);
    let directions = Some(vec![Direction::E, Direction::SE, Direction::S]);
    // let directions = Some(vec![Direction::NE, Direction::E, Direction::SE, Direction::S]);
    // let directions = Some(vec![Direction::NW]);
    Puzzle::find_best_puzzle(&slice_str_to_strings(&words.to_vec()), expansion, directions);
}
