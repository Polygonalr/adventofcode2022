use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use core::cmp::{min, max};

const GRID_HEIGHT: usize = 200;
const GRID_LENGTH: usize = 800;
const SAND_SPAWN_POINT: (usize, usize) = (500, 0);
const ABYSS_Y: usize = 199;

#[derive(Clone, PartialEq)]
enum Entity {
    ROCK,
    SAND,
    NOTHING,
}

fn build_grid(line_vec: &Vec<String>, with_floor: bool) -> Vec<Vec<Entity>> {
    let mut grid: Vec<Vec<Entity>> = vec![vec![Entity::NOTHING; GRID_HEIGHT]; GRID_LENGTH];
    let mut highest_y = 0;
    for line in line_vec {
        let s: Vec<(i32, i32)> = line.split(" -> ").map(|coords| {
            let coords_str_split: Vec<i32> = coords.split(',')
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect();
            (coords_str_split[0], coords_str_split[1])
        }).collect();
        let mut curr_coords = (s[0].0, s[0].1);
        highest_y = max(highest_y, s[0].1);
        for coords in s.into_iter().skip(1) {
            if curr_coords.0 != coords.0 { // change in x value
                assert!(curr_coords.1 == coords.1);
                let from = min(curr_coords.0, coords.0);
                let to = max(curr_coords.0, coords.0);
                for i in from..=to {
                    grid[i as usize][coords.1 as usize] = Entity::ROCK;
                }
            } else if curr_coords.1 != coords.1 { // change in y value
                assert!(curr_coords.0 == coords.0);
                let from = min(curr_coords.1, coords.1);
                let to = max(curr_coords.1, coords.1);
                for i in from..=to {
                    grid[coords.0 as usize][i as usize] = Entity::ROCK;
                }
                highest_y = max(highest_y, coords.1);
            }
            curr_coords = coords;
        }
    }
    if with_floor {
        for i in 0..GRID_LENGTH {
            grid[i][highest_y as usize + 2] = Entity::ROCK;
        }
    }
    grid
}

fn p1(grid: &mut Vec<Vec<Entity>>) -> i32 {
    let mut sand_count = 0;
    loop {
        sand_count += 1;
        let mut sand_coords: (usize, usize) = SAND_SPAWN_POINT;
        loop {
            if sand_coords.1 == ABYSS_Y {
                return sand_count - 1;
            }
            match grid[sand_coords.0][sand_coords.1 + 1] {
                Entity::NOTHING => {
                    sand_coords.1 += 1;
                }
                _ => { // something solid
                    if grid[sand_coords.0 - 1][sand_coords.1 + 1] == Entity::NOTHING { // check left first
                        sand_coords.0 -= 1;
                        sand_coords.1 += 1;
                    } else if grid[sand_coords.0 + 1][sand_coords.1 + 1] == Entity::NOTHING { // then check right
                        sand_coords.0 += 1;
                        sand_coords.1 += 1;
                    } else {
                        // settle the sand
                        grid[sand_coords.0][sand_coords.1] = Entity::SAND;
                        break;
                    }
                }
            }
        }
    }
}

// First part of the algorithm is exactly the same as part 1,
// except that we stop when we can no longer drop any sand at (500,0)
fn p2(grid: &mut Vec<Vec<Entity>>) -> i32 {
    let mut sand_count = 0;
    loop {
        if grid[SAND_SPAWN_POINT.0][SAND_SPAWN_POINT.1] == Entity::SAND {
            // cannot spawn any more sand
            return sand_count;
        }
        sand_count += 1;
        let mut sand_coords: (usize, usize) = SAND_SPAWN_POINT;
        loop {
            match grid[sand_coords.0][sand_coords.1 + 1] {
                Entity::NOTHING => {
                    sand_coords.1 += 1;
                }
                _ => { // something solid
                    if grid[sand_coords.0 - 1][sand_coords.1 + 1] == Entity::NOTHING { // check left first
                        sand_coords.0 -= 1;
                        sand_coords.1 += 1;
                    } else if grid[sand_coords.0 + 1][sand_coords.1 + 1] == Entity::NOTHING { // then check right
                        sand_coords.0 += 1;
                        sand_coords.1 += 1;
                    } else {
                        // settle the sand
                        grid[sand_coords.0][sand_coords.1] = Entity::SAND;
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    let filepath = "./input.txt";
    // let mut str_buf = "".to_owned();
    let mut line_vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines.flatten() {
            // Process each line...
            line_vec.push(line);
        }
    }
    let mut grid = build_grid(&line_vec, false);
    let mut grid2 =  build_grid(&line_vec, true);
    println!("Part 1: {}\nPart 2: {}", p1(&mut grid), p2(&mut grid2));
}

// Reusable function to read files
// From: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
