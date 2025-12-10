use std::thread;

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

#[allow(clippy::needless_range_loop)] // Range loops are easier to read for me than iter() + skip()
fn part_2(red_tiles: Vec<Coordinate>) {
    let (height, width) = calculate_field_size(&red_tiles);

    let mut field = allocate_field(height, width);

    insert_red_and_connecting_tiles(&red_tiles, &mut field);

    fill_outlined_shape(&mut field);

    let rects = calculate_rectangles(red_tiles);

    check_rects(field, rects);
}

fn calculate_field_size(tiles: &[Coordinate]) -> (usize, usize) {
    let height = tiles.iter().map(|tile| tile.row).max().unwrap() + 1;
    let width = tiles.iter().map(|tile| tile.col).max().unwrap() + 1;
    (height, width)
}

fn check_rects(field: Vec<Vec<bool>>, mut rects: Vec<Rectangle>) {
    println!("Checking rects");

    let mut biggest_rect_found = 0;

    thread::scope(|scope| {
        let field_r = &*field;

        let mut threads = Vec::with_capacity(MAX_THREADS);

        for _ in 0..MAX_THREADS.min(rects.len()) {
            let rectangle = rects.pop().unwrap();

            threads.push(scope.spawn(move || check_rect(field_r, rectangle)));
        }

        let mut current_thread = 0;
        let mut finished_threads = 0;

        while !threads.is_empty() {
            current_thread %= threads.len();
            if threads[current_thread].is_finished() {
                finished_threads += 1;

                if finished_threads % 100 == 0 {
                    println!(
                        "Result {finished_threads} in, still waiting for scheduling: {} rects",
                        rects.len()
                    );
                }

                let join_handle = if let Some(rect) = rects.pop()
                    && rect.size > biggest_rect_found
                {
                    
                    let mut swapper = scope.spawn(move || check_rect(field_r, rect));
                    std::mem::swap(&mut threads[current_thread], &mut swapper);
                    swapper
                } else {
                    threads.remove(current_thread)
                };

                if let Some(res) = join_handle.join().unwrap()
                    && res > biggest_rect_found
                {
                    biggest_rect_found = res;

                    println!("Found a new biggest size: {res}");
                }
            }
            current_thread += 1;
        }

        println!("Overall biggest rect: {biggest_rect_found}");
    });
}

fn calculate_rectangles(mut red_tiles: Vec<Coordinate>) -> Vec<Rectangle> {
    println!("Calculating rects");

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

#[allow(clippy::needless_range_loop)] // Range loops are easier to read for me than iter() + skip()
fn fill_outlined_shape(field: &mut [Vec<bool>]) {
    println!("Filling inside");

    for row_index in 0..field.len() {
        let mut fill = false;
        let mut continuuous_section = false;

        for col_index in 0..field[0].len() {
            if !fill && !field[row_index][col_index] {
                continue;
            } else if fill && !field[row_index][col_index] {
                if continuuous_section {
                    if row_index != 0 && field[row_index - 1][col_index] {
                        field[row_index][col_index] = true;
                        continuuous_section = false;
                    }
                } else {
                    field[row_index][col_index] = true;
                }
            } else if fill && field[row_index][col_index] {
                if !continuuous_section {
                    fill = false
                }
            } else if !fill && field[row_index][col_index] {
                fill = true;
                continuuous_section = true;
            }
        }
    }
}

#[allow(clippy::needless_range_loop)] // Range loops are easier to read for me than iter() + skip()
fn insert_red_and_connecting_tiles(red_tiles: &[Coordinate], field: &mut [Vec<bool>]) {
    println!("inserting red tiles and connection tiles");

    let mut last_tile: Option<Coordinate> = None;
    for current_tile in red_tiles {
        if let Some(last_tile) = last_tile {
            for row in last_tile.row.min(current_tile.row)..=last_tile.row.max(current_tile.row) {
                for col in last_tile.col.min(current_tile.col)..=last_tile.col.max(current_tile.col)
                {
                    field[row][col] = true;
                }
            }
        } else {
            field[current_tile.row][current_tile.col] = true;
        }
        last_tile = Some(*current_tile);
    }

    // make the last line end->start tile
    for row in
        last_tile.unwrap().row.min(red_tiles[0].row)..=last_tile.unwrap().row.max(red_tiles[0].row)
    {
        for col in last_tile.unwrap().col.min(red_tiles[0].col)
            ..=last_tile.unwrap().col.max(red_tiles[0].col)
        {
            field[row][col] = true;
        }
    }
}

#[allow(clippy::same_item_push)]
// We need to tightly control the size of our vectors. I could not find any guarantees regarding the allocation size of the vector when doing what this lint suggests.
fn allocate_field(height: usize, width: usize) -> Vec<Vec<bool>> {
    println!("Allocating field");
    let mut field = Vec::new();
    field.reserve_exact(height);
    for _ in 0..height {
        let mut line = Vec::new();
        line.reserve_exact(width);
        for _ in 0..width {
            line.push(false);
        }
        field.push(line);
    }
    field
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

#[allow(clippy::needless_range_loop)] // readability
fn check_rect(field: &[Vec<bool>], rect: Rectangle) -> Option<usize> {
    for row in rect.upper_left.row..=rect.lower_right.row {
        for col in rect.upper_left.col..=rect.lower_right.col {
            if !field[row][col] {
                return None;
            }
        }
    }
    Some(rect.size)
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
