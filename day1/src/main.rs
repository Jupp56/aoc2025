use std::time::Instant;

fn main() {
    let input = include_str!("../input/real");

    let lines: Vec<&str> = input.lines().collect();

    let start = Instant::now();
    //  part_1(&input);
    let p1 = Instant::now();
    let r2 = part_2_mit_change(lines);
    let p2 = Instant::now();

    println!("r2: {r2}");
    println!(
        "Time part 1: {} mics, time part 2: {} ns",
        (p1 - start).as_micros(),
        (p2 - p1).as_nanos()
    );
}

fn part_1(input: &str) {
    let mut dial = 50;

    let mut zeroes = 0;

    for line in input.lines() {
        let number = str::parse::<i16>(&line[1..]).unwrap();
        match line.chars().next().unwrap() {
            'R' => {
                dial += number;
            }
            'L' => dial -= number,
            _ => unimplemented!(),
        }

        dial %= 100;

        if dial < 0 {
            dial += 100;
        }

        if dial == 100 {
            dial = 0;
        }

        if dial == 0 {
            zeroes += 1
        }
    }

    println!("Encountered {zeroes} zeroes in part 1");
}

// ca. 48 mys
fn part_2_optimize_small_branchless(input: Vec<&str>) -> i16 {
    let mut dial: i16 = 50;

    let mut zeroes: i16 = 0;

    for line in input {
        let mut steps_to_go: i16 = str::parse::<i16>(&line[1..]).unwrap();

        zeroes += steps_to_go / 100;
        steps_to_go %= 100;

        let clockwise: bool = &line[0..1] == "R";

        let go_counterclockwise: i16 = !clockwise as i16;
        let go_clockwise: i16 = clockwise as i16;

        let clockwise_jumped_over_100: i16 = (dial + steps_to_go > 99) as i16;
        zeroes += go_clockwise * clockwise_jumped_over_100;
        dial -= go_clockwise * clockwise_jumped_over_100 * 100;
        dial += go_clockwise * steps_to_go;

        let count_counterclockwise: i16 = (dial != 0 && dial - steps_to_go <= 0) as i16;
        zeroes += go_counterclockwise * count_counterclockwise;

        let jump_counterclockwise = (dial - steps_to_go < 0) as i16;
        dial += go_counterclockwise * jump_counterclockwise * 100;
        dial -= go_counterclockwise * steps_to_go;
    }

    zeroes
}

// ca. 55 mys
fn part_2_optimize_small(input: Vec<&str>) -> i16 {
    let mut dial: i16 = 50;

    let mut zeroes: i16 = 0;

    for line in input {
        let mut steps_to_go: i16 = str::parse::<i16>(&line[1..]).unwrap();

        let clockwise: bool = &line[0..1] == "R";

        zeroes += steps_to_go / 100;
        steps_to_go %= 100;

        if clockwise {
            if dial + steps_to_go > 99 {
                zeroes += 1;
                dial -= 100;
            }

            dial += steps_to_go;
        } else {
            if dial != 0 && dial - steps_to_go <= 0 {
                zeroes += 1;
            }
            if dial - steps_to_go < 0 {
                dial += 100;
            }

            dial -= steps_to_go;
        }
    }

    zeroes
}

// ca. 380 mys
fn part_2_optimize_big_jumps(input: Vec<&str>) -> i16 {
    let mut dial = 50;

    let mut zeroes = 0;

    for line in input {
        let mut steps_to_go = str::parse::<i16>(&line[1..]).unwrap();

        let clockwise = &line[0..1] == "R";

        zeroes += steps_to_go / 100;
        steps_to_go %= 100;

        while steps_to_go != 0 {
            if dial == 0 {
                zeroes += 1;
            }

            steps_to_go -= 1;

            if clockwise {
                dial += 1;
            } else {
                dial -= 1;
            }

            if dial < 0 {
                dial = 99;
            } else if dial >= 100 {
                dial = 0;
            }
        }
    }

    zeroes
}

// ca. 1500 mys
fn part_2_branchless(input: Vec<&str>) -> i32 {
    let mut dial = 50;

    let mut zeroes = 0;

    for line in input {
        let mut steps_to_go = str::parse::<i16>(&line[1..]).unwrap();

        let clockwise = &line[0..1] == "R";

        while steps_to_go != 0 {
            // println!("{dial}, {steps_to_go}, {line}");

            zeroes += (dial == 0) as i32;

            steps_to_go -= 1;

            dial += clockwise as i16 + -(!clockwise as i16);

            dial = (dial >= 0) as i16 * dial + (dial < 0) as i16 * 99;
            dial *= (dial < 100) as i16;
        }
    }

    zeroes
}

// ca.1000 mys
fn part_2_branch_ende(input: Vec<&str>) -> i32 {
    let mut dial = 50;

    let mut zeroes = 0;

    for line in input {
        let mut steps_to_go = str::parse::<i16>(&line[1..]).unwrap();

        let clockwise = &line[0..1] == "R";

        while steps_to_go != 0 {
            if dial == 0 {
                zeroes += 1;
            }

            steps_to_go -= 1;

            if clockwise {
                dial += 1;
            } else {
                dial -= 1;
            }

            if dial < 0 {
                dial = 99;
            } else if dial >= 100 {
                dial = 0;
            }
        }
    }

    zeroes
}

// ca. 1050 mys
fn part_2_changeberechnung_branchless(input: Vec<&str>) -> i32 {
    let mut dial = 50;

    let mut zeroes = 0;

    for line in input {
        let number = str::parse::<i16>(&line[1..]).unwrap();

        let mut change = number * (&line[0..1] == "L") as i16;

        while change != 0 {
            if dial == 0 {
                zeroes += 1;
            }

            if change > 0 {
                change -= 1;
                dial -= 1;
            } else if change < 0 {
                change += 1;
                dial += 1;
            }

            if dial < 0 {
                dial = 99;
            } else if dial >= 100 {
                dial = 0;
            }
        }
    }
    zeroes
}

// ca 2100 mys
fn part_2_mit_change(input: Vec<&str>) -> i32 {
    let mut dial = 50;

    let mut zeroes = 0;

    for line in input {
        let mut change;

        let number = str::parse::<i16>(&line[1..]).unwrap();

        match &line[0..1] {
            "R" => {
                change = number;
            }
            "L" => change = -number,
            _ => unimplemented!(),
        }

        while change != 0 {
            if dial == 0 {
                zeroes += 1;
            }

            if change > 0 {
                change -= 1;
                dial -= 1;
            } else if change < 0 {
                change += 1;
                dial += 1;
            }

            if dial < 0 {
                dial = 99;
            } else if dial >= 100 {
                dial = 0;
            }
        }
    }

    zeroes
}
