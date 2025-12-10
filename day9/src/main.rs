fn main() {

    let input = include_str!("../input/real");

    let lines: Vec<&str> = input.lines().collect();

    let mut reds = Vec::new();

    for line in lines {
        let mut s = line.split(',');
        let first = str::parse::<u64>(s.next().unwrap()).unwrap();
        let second = str::parse::<u64>(s.next().unwrap()).unwrap();

        reds.push((first, second));
    }

    let mut max = 0;

    for square in &reds {
        for square_2 in reds.iter().filter(|x| *x != square) {
            let height = square.0.abs_diff(square_2.0) + 1;
            let width = square.1.abs_diff(square_2.1) + 1;

            max = max.max(height * width);
        }
    }

    println!("max: {max}");
}
