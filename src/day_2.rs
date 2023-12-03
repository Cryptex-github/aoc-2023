use std::fs::read_to_string;

pub fn p1() {
    let input = read_to_string("./inputs/day-2.txt").unwrap();

    let total = input.lines().map(|game| {
        let (first, last) = game.split_once(": ").unwrap();
        let game_id = first[5..].parse::<u32>().unwrap();
        let mut possible = true;

        let sets = last.split("; ");

        for set in sets {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let cubes = set.split(", ");
            for cube in cubes {
                let (count, color) = cube.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();
                
                match color {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => {}
                }
            }

            if red > 12 || green > 13 || blue > 14 && possible {
                possible = false;
            }
        }

        if possible {
            game_id
        } else { 0 }
    })
    .sum::<u32>();

    println!("{total}")
}

pub fn p2() {
    let input = read_to_string("./inputs/day-2.txt").unwrap();

    let total = input.lines().map(|game| {
        let (_, last) = game.split_once(": ").unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let sets = last.split("; ");

        for set in sets {
            let cubes = set.split(", ");
            for cube in cubes {
                let (count, color) = cube.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();
                
                match color {
                    "red" => if count > red { red = count; },
                    "green" => if count > green { green = count; },
                    "blue" => if count > blue { blue = count; },
                    _ => {}
                }
            }
        }

        red * green * blue
    })
    .sum::<u32>();

    println!("{total}")
}