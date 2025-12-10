use std::thread;

use tightvec::TightVec;

const MAX_THREADS: usize = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rectangle {
    upper_left: Coordinate,
    lower_right: Coordinate,
    size: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    row: usize,
    col: usize,
}

fn main() {
    let input = include_str!("../input/real");

    let lines: Vec<&str> = input.lines().collect();

    let reds = parse_red_tiles(lines);

    part_1(&reds);

    part_2(reds);
}

fn parse_red_tiles(lines: Vec<&str>) -> Vec<Coordinate> {
    let mut reds = Vec::new();

    for line in lines {
        let mut s = line.split(',');
        let first = str::parse::<usize>(s.next().unwrap()).unwrap();
        let second = str::parse::<usize>(s.next().unwrap()).unwrap();

        reds.push(Coordinate {
            col: first,
            row: second,
        });
    }
    reds
}

fn part_1(red_tiles: &[Coordinate]) {
    let mut max = 0;

    for square in red_tiles {
        for square_2 in red_tiles.iter().filter(|x| *x != square) {
            let height = square.row.abs_diff(square_2.row) + 1;
            let width = square.col.abs_diff(square_2.col) + 1;

            max = max.max(height * width);
        }
    }

    println!("Part 1: {max}");
}

#[allow(clippy::needless_range_loop)] // Range loops are easier to read for me than iter() + skip()
fn part_2(red_tiles: Vec<Coordinate>) {
    let (height, width) = calculate_field_size(&red_tiles);

    let mut field = allocate_field(height, width);

    insert_red_and_connecting_tiles(&red_tiles, &mut field);

    fill_outlined_shape(&mut field);

    let rects = calculate_possible_rectangles(red_tiles);

    check_rectangles(&field, rects);
}

fn calculate_field_size(tiles: &[Coordinate]) -> (usize, usize) {
    let height = tiles.iter().map(|tile| tile.row).max().unwrap() + 1;
    let width = tiles.iter().map(|tile| tile.col).max().unwrap() + 1;
    (height, width)
}

fn allocate_field(height: usize, width: usize) -> Vec<TightVec> {
    println!("Allocating field");
    let mut field = Vec::with_capacity(height);

    let line = TightVec::with_len_and_value(width, false);
    for _ in 0..height {
        field.push(line.clone());
    }
    field
}

#[allow(clippy::needless_range_loop)] // Range loops are easier to read for me than iter() + skip()
fn insert_red_and_connecting_tiles(red_tiles: &[Coordinate], field: &mut [TightVec]) {
    println!("Inserting red tiles and connecting tiles");

    let mut last_tile: Option<Coordinate> = None;
    for current_tile in red_tiles {
        if let Some(last_tile) = last_tile {
            for row in last_tile.row.min(current_tile.row)..=last_tile.row.max(current_tile.row) {
                for col in last_tile.col.min(current_tile.col)..=last_tile.col.max(current_tile.col)
                {
                    field[row].set(col, true);
                }
            }
        } else {
            field[current_tile.row].set(current_tile.col, true);
        }
        last_tile = Some(*current_tile);
    }

    // make the last line end->start tile
    let first_tile = red_tiles[0];
    let last_tile = last_tile.unwrap();

    for row in last_tile.row.min(first_tile.row)..=last_tile.row.max(first_tile.row) {
        for col in last_tile.col.min(first_tile.col)..=last_tile.col.max(first_tile.col) {
            field[row].set(col, true);
        }
    }
}

fn calculate_possible_rectangles(mut red_tiles: Vec<Coordinate>) -> Vec<Rectangle> {
    println!("Calculating possible rectangles");

    let mut rects = Vec::new();

    while let Some(tile) = red_tiles.pop() {
        for other_tile in &red_tiles {
            let height = tile.row.abs_diff(other_tile.row) + 1;
            let width = tile.col.abs_diff(other_tile.col) + 1;

            rects.push(Rectangle {
                upper_left: Coordinate {
                    row: tile.row.min(other_tile.row),
                    col: tile.col.min(other_tile.col),
                },
                lower_right: Coordinate {
                    row: tile.row.max(other_tile.row),
                    col: tile.col.max(other_tile.col),
                },
                size: height * width,
            });
        }
    }

    rects.sort_by(|r1, r2| r1.size.cmp(&r2.size));
    rects
}

fn check_rectangles(field: &[TightVec], mut rects: Vec<Rectangle>) {
    println!("Checking rectangles");

    let mut biggest_rect_found = 0;

    thread::scope(|scope| {
        let mut threads = Vec::with_capacity(MAX_THREADS);

        for _ in 0..MAX_THREADS.min(rects.len()) {
            let rectangle = rects.pop().unwrap();

            threads.push(scope.spawn(move || check_rectangle(field, rectangle)));
        }

        let mut current_thread = 0;
        let mut finished_threads = 0;

        while !threads.is_empty() {
            current_thread %= threads.len();
            if threads[current_thread].is_finished() {
                finished_threads += 1;

                if finished_threads % 1000 == 0 {
                    println!(
                        "Result {finished_threads} in, still waiting for scheduling: {} rects",
                        rects.len()
                    );
                }

                let join_handle = if let Some(rect) = rects.pop()
                    && rect.size > biggest_rect_found
                {
                    let mut swapper = scope.spawn(move || check_rectangle(field, rect));
                    std::mem::swap(&mut threads[current_thread], &mut swapper);
                    swapper
                } else {
                    threads.remove(current_thread)
                };

                if let Some(res) = join_handle.join().unwrap()
                    && res > biggest_rect_found
                {
                    biggest_rect_found = res;

                    println!("Found a new biggest rectangle candidate: {res}");
                }
            }
            current_thread += 1;
        }

        println!("Part 2: Overall biggest rectangle: {biggest_rect_found}");
    });
}

#[allow(clippy::needless_range_loop)] // Range loops are easier to read for me than iter() + skip()
fn fill_outlined_shape(field: &mut [TightVec]) {
    println!("Filling area");

    for row_index in 0..field.len() {
        let mut fill = false;
        let mut continuuous_section = false;
        let mut uninterrupted_start = 0;

        for col_index in 0..field[0].len() {
            if !fill && !field[row_index].index(col_index) {
                continue;
            } else if fill && !field[row_index].index(col_index) {
                if continuuous_section {
                    if row_index != 0 && field[row_index - 1].index(col_index) {
                        continuuous_section = false;
                    } else {
                        field[row_index].fill_multiple(uninterrupted_start, col_index - 1, true);
                    }
                }
            } else if fill && field[row_index].index(col_index) {
                if !continuuous_section {
                    field[row_index].fill_multiple(uninterrupted_start, col_index - 1, true);
                    fill = false
                }
            } else if !fill && field[row_index].index(col_index) {
                uninterrupted_start = col_index;
                fill = true;
                continuuous_section = true;
            }
        }
    }
}

#[allow(clippy::needless_range_loop)] // readability
fn check_rectangle(field: &[TightVec], rect: Rectangle) -> Option<usize> {
    const REGISTER_WIDTH: usize = 64;

    for row in rect.upper_left.row..=rect.lower_right.row {
        let mut current = rect.upper_left.col;

        while !current.is_multiple_of(REGISTER_WIDTH) && current < rect.lower_right.col {
            if !field[row].index(current) {
                return None;
            }
            current += 1;
        }

        while rect.lower_right.col - current > REGISTER_WIDTH {
            if field[row].get_raw()[current / 64] != u64::MAX {
                return None;
            }
            current += REGISTER_WIDTH;
        }

        for col in current..=rect.lower_right.col {
            if !field[row].index(col) {
                return None;
            }
        }
    }
    Some(rect.size)
}
