
use std::{io::{BufReader, BufRead}, fs::File, collections::{VecDeque, HashMap, HashSet}};

const BACKSPACE: char = 8u8 as char;

const TOTAL_ROCKS_LONG: usize = 1000000000000;

const TOTAL_ROCKS_SHORT: usize = 2022;

const TOTAL_ROCKS: usize = TOTAL_ROCKS_LONG;

enum Direction {
    LEFT,
    RIGHT
}

struct Arena {
    field: VecDeque<Vec<bool>>
}

struct Rock {
    x: usize,
    y: usize,
    shape: RockType
}

struct ArenaAnalytics {
    row_meta: Vec<(usize, RockType, usize)>
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RockType{
    MINUS,
    PLUS,
    REVERSE_L,
    COLUMN,
    SQUARE
}

impl Arena {

    fn find_overlaps(&self, rock: &Rock) -> bool {

        if rock.y == 0 {
            return false;
        }

        if rock.y == self.field.len() + 1 {
            return true;
        }

        let rock_field = rock.get_field();

        for (arena_row_i, rock_row_i) in 
        (std::cmp::max(0, rock.y as i32 - rock_field.len() as i32) as usize .. rock.y).zip(
            std::cmp::max(0, rock_field.len() as i32 - rock.y as i32) as usize .. rock_field.len()) {

            let arena_row = &self.field[arena_row_i];
            let rock_row = &rock_field[rock_row_i];

            for (arena_column, rock_column) in arena_row.iter().zip(rock_row.iter()) {
                if *arena_column && *rock_column {
                    return true;
                }
            }
        }

        false
    }

    fn add_rock(&mut self, rock: &Rock) {

        let rock_field = rock.get_field();

        self.field.push_front(vec![false, false, false, false, false, false, false]);
        
        for (overlapping_row_arena_i, overlapping_row_rock_i) in 
        (std::cmp::max(0, rock.y as i32 - rock_field.len() as i32) as usize .. rock.y  + 1).zip(
            std::cmp::max(0, rock_field.len() as i32 - rock.y as i32) as usize .. rock_field.len()) {

            let overlapping_arena_row = &mut self.field[overlapping_row_arena_i];
            let overlapping_rock_row = &rock_field[overlapping_row_rock_i];

            for (element_arena, element_rock) in overlapping_arena_row.iter_mut().zip(overlapping_rock_row.iter()) {
                if *element_rock {
                    *element_arena = true;
                }
            }
        }

        let mut last_empty_row = self.field.len();

        for (row_i, row) in self.field.iter().enumerate() {
            if row.contains(&true) {
                last_empty_row = row_i;
                break;
            }
        }

        for _ in last_empty_row..3 {
            self.field.push_front(vec![false, false, false, false, false, false, false]);
        }

        for _ in 3..last_empty_row {
            self.field.pop_front();
        }
    }

    fn print(&self) {
        for row in &self.field {
            println!("|{}|", row.iter().map(|&e| if e { '#' } else { '.' }).collect::<String>());
        }

        println!("+-------+\n");
    }
}

impl ArenaAnalytics {

    fn analyze_row(&mut self, arena: &VecDeque<Vec<bool>>, new_rock_index: usize, rock_type: RockType, jet_input: usize) -> (bool, usize) {

        let search_result: Vec<(usize, usize)> = self.row_meta.iter().enumerate().filter_map(
            |(i, v)| if v.1 == rock_type && v.2 == jet_input {Some((i, v.0))} else {None}).collect::<Vec<(usize, usize)>>();

        let new_arena_index: usize = arena.len() - 3;

        self.row_meta.insert(new_rock_index, (new_arena_index, rock_type, jet_input));

        for (rock_index, arena_index)  in search_result.iter().rev() {
            if arena_index < &(new_arena_index / 2) {
                break;
            } else if !search_result.iter().any(|(_, ta)| ta == &(2 * arena_index - new_arena_index)) {
                continue;
            }

            let mut res = true;

            for (lower_rock_index, higher_rock_index) in (2* (*rock_index)- (new_rock_index)..*rock_index).zip(*rock_index..new_rock_index) {
                if self.row_meta[lower_rock_index].1 != self.row_meta[higher_rock_index].1 
                || self.row_meta[lower_rock_index].2 != self.row_meta[higher_rock_index].2 {
                    res = false;
                    break;
                }

            }

            if res {
                println!("found something interesting!!! [{}, {}, {}] lengths [{}, {}]",
                2* (*rock_index)- (new_rock_index), *rock_index, new_rock_index,
                *rock_index - (2* (*rock_index)- (new_rock_index)), new_rock_index - *rock_index);
                println!("arena heights [{}, {}, {}] lengths [{}, {}]",
                self.row_meta[2* (*rock_index)- (new_rock_index)].0, *arena_index, new_arena_index,
                *arena_index - (self.row_meta[2* (*rock_index)- (new_rock_index)].0), new_arena_index - *arena_index);


                 let pattern_len_arena = (new_arena_index - arena_index);
                 let pattern_len_rock_index = new_rock_index - *rock_index;
                 let first_index_arena = new_arena_index - 2 * pattern_len_arena;
                 let first_index_rock = new_rock_index - 2 * pattern_len_rock_index;
                 let number_of_patterns = (TOTAL_ROCKS - first_index_rock) / pattern_len_rock_index;
                 let last_pattern_index_rock = first_index_rock + number_of_patterns * pattern_len_rock_index;
                 let remainder = TOTAL_ROCKS - last_pattern_index_rock;
                 let last_height_pattern = first_index_arena + number_of_patterns * pattern_len_arena;

                 let height_remainder = self.row_meta[first_index_rock + remainder - 1].0 - first_index_arena;

                 let total_height = height_remainder + last_height_pattern;

                 return (false, 0);
                 //return (true, last_height_pattern + height_remainder);
            }

        }

        (false, 0)
    }


}

fn detect_pattern(arena: &VecDeque<Vec<bool>>, pivot_point: usize) -> bool {

    let end_second_range = arena.len() - 3;
    let begin_first_range = 2 * pivot_point - end_second_range;

    for (row_i, row_j) in (begin_first_range..pivot_point).zip(pivot_point..end_second_range) {
        
        for (a, b) in arena[row_i].iter().zip(arena[row_j].iter()){
            if *a != *b {
                return false;
            }
        }
    }

    true
}

impl Rock {

    const MINUS_0: [[bool; 7]; 1] = [[true, true, true, true, false, false, false]];
    const MINUS_1: [[bool; 7]; 1] = [[false, true, true, true, true, false, false]];
    const MINUS_2: [[bool; 7]; 1] = [[false, false, true, true, true, true, false]];
    const MINUS_3: [[bool; 7]; 1] = [[false, false, false, true, true, true, true]];

    const PLUS_0: [[bool; 7]; 3] = [
            [false, true, false, false, false, false, false], 
            [true, true, true, false, false, false, false], 
            [false, true, false, false, false, false, false]
            ];

    const PLUS_1: [[bool; 7]; 3] = [
            [false, false, true, false, false, false, false], 
            [false, true, true, true, false, false, false], 
            [false, false, true, false, false, false, false]
            ];
    
    const PLUS_2: [[bool; 7]; 3] = [
            [false, false, false, true, false, false, false], 
            [false, false, true, true, true, false, false], 
            [false, false, false, true, false, false, false]
            ];
    
    const PLUS_3: [[bool; 7]; 3] = [
            [false, false, false, false, true, false, false], 
            [false, false, false, true, true, true, false], 
            [false, false, false, false, true, false, false]
            ];

    const PLUS_4: [[bool; 7]; 3] = [
            [false, false, false, false, false, true, false], 
            [false, false, false, false, true, true, true], 
            [false, false, false, false, false, true, false]
            ];

    const REVERSE_L_0: [[bool; 7]; 3] = [
            [false, false, true, false, false, false, false], 
            [false, false, true, false, false, false, false], 
            [true, true, true, false, false, false, false]
            ];

    const REVERSE_L_1: [[bool; 7]; 3] = [
            [false, false, false, true, false, false, false], 
            [false, false, false, true, false, false, false], 
            [false, true, true, true, false, false, false]
            ];

    const REVERSE_L_2: [[bool; 7]; 3] = [
            [false, false, false, false, true, false, false],
            [false, false, false, false, true, false, false],
            [false, false, true, true, true, false, false]
            ];

    const REVERSE_L_3: [[bool; 7]; 3] = [
            [false, false, false, false, false, true, false], 
            [false, false, false, false, false, true, false], 
            [false, false, false, true, true, true, false]
            ];

    const REVERSE_L_4: [[bool; 7]; 3] = [
            [false, false, false, false, false, false, true], 
            [false, false, false, false, false, false, true], 
            [false, false, false, false, true, true, true]
            ];

    const COLUMN_0: [[bool; 7]; 4] = [
            [true, false, false, false, false, false, false], 
            [true, false, false, false, false, false, false], 
            [true, false, false, false, false, false, false], 
            [true, false, false, false, false, false, false]
            ];

    const COLUMN_1: [[bool; 7]; 4] = [
            [false, true, false, false, false, false, false], 
            [false, true, false, false, false, false, false], 
            [false, true, false, false, false, false, false], 
            [false, true, false, false, false, false, false]
            ];

    const COLUMN_2: [[bool; 7]; 4] = [
            [false, false, true, false, false, false, false], 
            [false, false, true, false, false, false, false], 
            [false, false, true, false, false, false, false], 
            [false, false, true, false, false, false, false]
            ];

    const COLUMN_3: [[bool; 7]; 4] = [
            [false, false, false, true, false, false, false], 
            [false, false, false, true, false, false, false], 
            [false, false, false, true, false, false, false], 
            [false, false, false, true, false, false, false]
            ];

    const COLUMN_4: [[bool; 7]; 4] = [
            [false, false, false, false, true, false, false], 
            [false, false, false, false, true, false, false], 
            [false, false, false, false, true, false, false], 
            [false, false, false, false, true, false, false]
            ];

    const COLUMN_5: [[bool; 7]; 4] = [
            [false, false, false, false, false, true, false], 
            [false, false, false, false, false, true, false], 
            [false, false, false, false, false, true, false], 
            [false, false, false, false, false, true, false]
            ];

    const COLUMN_6: [[bool; 7]; 4] = [
            [false, false, false, false, false, false, true], 
            [false, false, false, false, false, false, true], 
            [false, false, false, false, false, false, true], 
            [false, false, false, false, false, false, true]
            ];

    const SQUARE_0: [[bool; 7]; 2] = [
            [true, true, false, false, false, false, false], 
            [true, true, false, false, false, false, false]
            ];

    const SQUARE_1: [[bool; 7]; 2] = [
            [false, true, true, false, false, false, false], 
            [false, true, true, false, false, false, false]
            ];

    const SQUARE_2: [[bool; 7]; 2] = [
            [false, false, true, true, false, false, false], 
            [false, false, true, true, false, false, false]
            ];

    const SQUARE_3: [[bool; 7]; 2] = [
            [false, false, false, true, true, false, false], 
            [false, false, false, true, true, false, false]
            ];

    const SQUARE_4: [[bool; 7]; 2] = [
            [false, false, false, false, true, true, false], 
            [false, false, false, false, true, true, false]
            ];

    const SQUARE_5: [[bool; 7]; 2] = [
            [false, false, false, false, false, true, true], 
            [false, false, false, false, false, true, true]
            ];

    fn from_u32(value: usize) -> Rock {
        match value {
            0 => Rock{x: 2, y: 0, shape: RockType::MINUS},
            1 => Rock{x: 2, y: 0, shape: RockType::PLUS},
            2 => Rock{x: 2, y: 0, shape: RockType::REVERSE_L},
            3 => Rock{x: 2, y: 0, shape: RockType::COLUMN},
            4 => Rock{x: 2, y: 0, shape: RockType::SQUARE},
            _ => panic!("Unknown value: {}", value),
        }
    }
    
    fn get_field(&self) -> &[[bool; 7]] {

        match self.shape {
            RockType::MINUS => {
                if self.x == 0 {
                    return &Rock::MINUS_0;
                } else if self.x == 1 {
                    return &Rock::MINUS_1;
                } else if self.x == 2 {
                    return &Rock::MINUS_2;
                } else {
                    return &Rock::MINUS_3;
                }
            },
            RockType::PLUS => {
                if self.x == 0 {
                    return &Rock::PLUS_0;
                } else if self.x == 1 {
                    return &Rock::PLUS_1;
                } else if self.x == 2 {
                    return &Rock::PLUS_2;
                } else if self.x == 3{
                    return &Rock::PLUS_3;
                } else {
                    return &Rock::PLUS_4;
                }
            },
            RockType::REVERSE_L => {
                if self.x == 0 {
                    return &Rock::REVERSE_L_0;
                } else if self.x == 1 {
                    return &Rock::REVERSE_L_1;
                } else if self.x == 2 {
                    return &Rock::REVERSE_L_2;
                } else if self.x == 3{
                    return &Rock::REVERSE_L_3;
                } else {
                    return &Rock::REVERSE_L_4;
                }
            },
            RockType::COLUMN => {
                if self.x == 0 {
                    return &Rock::COLUMN_0;
                } else if self.x == 1 {
                    return &Rock::COLUMN_1;
                } else if self.x == 2 {
                    return &Rock::COLUMN_2;
                } else if self.x == 3 {
                    return &Rock::COLUMN_3;
                } else if self.x == 4 {
                    return &Rock::COLUMN_4;
                } else if self.x == 5 {
                    return &Rock::COLUMN_5;
                } else {
                    return &Rock::COLUMN_6;
                }
            }
            RockType::SQUARE => {
                if self.x == 0 {
                    return &Rock::SQUARE_0;
                } else if self.x == 1 {
                    return &Rock::SQUARE_1;
                } else if self.x == 2 {
                    return &Rock::SQUARE_2;
                } else if self.x == 3 {
                    return &Rock::SQUARE_3;
                } else if self.x == 4 {
                    return &Rock::SQUARE_4;
                } else {
                    return &Rock::SQUARE_5;
                }
            }
        }
    }

}

fn simulate(jet_inputs: &mut Vec<Direction>) -> usize{

    let mut arena: Arena = Arena{field: VecDeque::from(
        vec![
            vec![false, false, false, false, false, false, false], 
            vec![false, false, false, false, false, false, false], 
            vec![false, false, false, false, false, false, false]
            ])};
    
    let mut arena_analyzer = ArenaAnalytics{row_meta: Vec::with_capacity(50000)};

    let mut input_index = 0;

    for rock_index in 0..TOTAL_ROCKS as usize {
        //print!("{}\ri: {} / TOTAL_ROCKS", BACKSPACE, rock_index);

        let mut rock = Rock::from_u32(rock_index.rem_euclid(5));
        let original_jet_input = input_index;

        drop_rock(&mut arena, &mut rock, jet_inputs, &mut input_index);

        let (pattern_detected, end_height) = arena_analyzer.analyze_row(&arena.field, rock_index,  rock.shape, original_jet_input);
        if pattern_detected {
            return end_height;
        }
    }

    return TOTAL_ROCKS;
}

fn drop_rock(arena: &mut Arena, rock: &mut Rock, jet_inputs: &Vec<Direction>, input_index: &mut usize) {

    while !arena.find_overlaps(&rock) {
        let input = &jet_inputs[*input_index];
        *input_index = (*input_index + 1).rem_euclid(jet_inputs.len());        

        match input {
            Direction::LEFT => {if rock.x > 0 {rock.x -= 1;}},
            Direction::RIGHT => {
                match rock.shape {
                    RockType::MINUS => {if rock.x < 3 {rock.x +=1;}},
                    RockType::PLUS => {if rock.x < 4 {rock.x +=1;}},
                    RockType::REVERSE_L => {if rock.x < 4 {rock.x +=1;}},
                    RockType::SQUARE => {if rock.x < 5 {rock.x +=1;}},
                    RockType::COLUMN => {if rock.x < 6 {rock.x +=1;}}
                }
            }
        }

        if arena.find_overlaps(&rock) {
            match input {
                Direction::LEFT =>{rock.x += 1;},
                Direction::RIGHT => {rock.x -= 1;}
            }
        }

        rock.y += 1;
    }

    arena.add_rock(&rock);
}

fn main() {

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let directions: Vec<Direction> = reader.lines().nth(0).unwrap().unwrap().chars().map(|c| if c=='<' {Direction::LEFT} else {Direction::RIGHT}).collect();

    println!("total length {}", simulate(&mut (directions.into_iter().collect())));    
}