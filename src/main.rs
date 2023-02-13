
use std::{io::{BufReader, BufRead}, fs::File, collections::{VecDeque, HashSet}};

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
    

    let mut curbed_arena = 0;
    let mut input_index = 0;

    for rock_index in 0..2022 as usize {
        drop_rock(&mut arena, Rock::from_u32(rock_index.rem_euclid(5)), jet_inputs, &mut input_index);
        //curbed_arena += curb(&mut arena);
        //arena.print();
    }

    let mut cropped = 0;
    for row in arena.field.iter() {
        if row.contains(&true) {
            break;
        }
        cropped += 1;
    }

    arena.print();

    curbed_arena + arena.field.len() - cropped
}

fn drop_rock(arena: &mut Arena, mut rock: Rock, jet_inputs: &Vec<Direction>, input_index: &mut usize) {

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

fn curb(arena: &mut Arena) -> usize {

    let mut result = false;
    let mut max_head = 0;

    for (i, _) in arena.field.iter().enumerate() {

        if !arena.field[i][0] {
            continue;
        } else {
            let mut heads: HashSet<usize> = HashSet::from([i]);

            for j in 1..6 {

                let mut new_heads: HashSet<usize> = HashSet::new();
                max_head = 0;
                
                for head in heads {
                    if head > 0 && arena.field[head - 1][j] {
                        new_heads.insert(head-1);
                        max_head = std::cmp::max(max_head, head -1);
                    }
                    
                    if arena.field[head][j] {
                        new_heads.insert(head);
                        max_head = std::cmp::max(max_head, head);
                    }

                    if head < arena.field.len() - 1 && arena.field[head + 1][j] {
                        new_heads.insert(head + 1);
                        max_head = std::cmp::max(max_head, head + 1);
                    }
                }

                if new_heads.is_empty() {
                    heads = new_heads;
                    break;
                }
                heads = new_heads;
            }

            if heads.is_empty() {
                continue;
            } else {
                result = true;
                break;
            }
        }
    }

    if result {

        let mut res = 0;

        for row in max_head + 1..arena.field.len() {
            arena.field.remove(row);
            res += 1;
        }

        return res;
    } else {
        return 0;
    }
}

fn main() {

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let directions: Vec<Direction> = reader.lines().nth(0).unwrap().unwrap().chars().map(|c| if c=='<' {Direction::LEFT} else {Direction::RIGHT}).collect();

    println!("total length {}", simulate(&mut (directions.into_iter().collect())));    
}