fn main() {
    let input = include_str!("../input/real");

    let lines: Vec<&str> = input.lines().collect();

    part_1(&lines);
    part_2(&lines);
}

fn part_1(lines: &[&str]) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    let mut switch_to_ingredients = false;

    for line in lines {
        if line.is_empty() {
            switch_to_ingredients = true;
            continue;
        }

        if !switch_to_ingredients {
            ranges.push(parse_range(line));
        } else {
            ingredients.push(str::parse::<u64>(line).unwrap());
        }
    }

    let unspoiled_ingredients: u64 = ingredients.iter().fold(0, |acc, ingredient| {
        acc + ranges.iter().any(|range| range.contains(ingredient)) as u64
    });

    println!("Ingredients: {unspoiled_ingredients}");
}

fn parse_range(line: &str) -> std::ops::Range<u64> {
    let mut split = line.split("-");

    str::parse(split.next().unwrap()).unwrap()
        ..str::parse::<u64>(split.next().unwrap()).unwrap() + 1
}


fn part_2(lines: &[&str]) {
    let mut ranges = Vec::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        ranges.push(parse_range_2(line));
    }

    let mut ranges_new: Vec<(u64, u64)> = Vec::new();

    'outer: while let Some(range) = ranges.pop() {
        let mut ranges_to_reconsider = Vec::new();
        let mut ranges_to_delete = Vec::new();

        for range_new in &mut ranges_new {
            // Anfang und Ende liegen drin
            if range_new.0 <= range.0 && range_new.1 >= range.1 {
                continue 'outer;
            } else
            // Anfang und Ende liegen drumrum
            if range.0 <= range_new.0 && range.1 >= range_new.1 {
                ranges_to_delete.push(*range_new);
                ranges_to_reconsider.push(range);
            } else
            // Anfang liegt drin
            if range_new.0 <= range.0 && range_new.1 >= range.0 {
                range_new.1 = range.1.max(range_new.1);
                ranges_to_reconsider.push(*range_new);
            }
            // Ende liegt drin
            else if range_new.0 <= range.1 && range_new.1 >= range.1 {
                range_new.0 = range.0.min(range_new.0);
                ranges_to_reconsider.push(*range_new);
            }
        }

        if ranges_to_reconsider.is_empty() && ranges_to_delete.is_empty() {
            ranges_new.push(range);
        }

        for range in ranges_to_delete {
            ranges_new.retain(|x| *x != range);
        }

        for range in ranges_to_reconsider {
            ranges.push(range);
            ranges_new.retain(|x| *x != range);
        }
    }

    let mut count = 0;
    for (low, high) in ranges_new {
        count += high + 1 - low;
    }
    println!("Unspoilt ingredients part 2: {count:}");
}

fn parse_range_2(line: &str) -> (u64, u64) {
    let mut split = line.split("-");

    (
        str::parse(split.next().unwrap()).unwrap(),
        str::parse::<u64>(split.next().unwrap()).unwrap(),
    )
}
