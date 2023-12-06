use std::{collections::HashSet, fs::read_to_string};

pub fn p1() {
    let input = read_to_string("./inputs/day-4.txt").unwrap();

    let cards = input
        .lines()
        .map(|l| l[10..].split_once(" | ").unwrap())
        .map(|(winning, have)| {
            winning
                .split_whitespace()
                .collect::<HashSet<_>>()
                .intersection(
                    &have
                        .split_whitespace()
                        .collect::<HashSet<_>>(),
                )
                .count()
        });

    let mut points = 0;

    for total_winning in cards {
        match total_winning {
            0 => {}
            1 => points += 1,
            _ => points += 2_usize.pow(total_winning as u32 - 1),
        }
    }

    println!("{points}");
}

pub fn p2() {
    let input = read_to_string("./inputs/day-4.txt").unwrap();
    let mut cards = vec![1; input.lines().count()];

    for (card_number, total_winning) in input.lines()
        .map(|l| {
            let (winning, have) = l[10..].split_once(" | ").unwrap();

            winning
                .split_whitespace()
                .collect::<HashSet<_>>()
                .intersection(
                    &have
                        .split_whitespace()
                        .collect::<HashSet<_>>(),
                )
                .count()
        })
        .enumerate()
    {
        for card in card_number + 1..=card_number + total_winning {
            cards[card] += cards[card_number]
        }
    }

    println!("{}", cards.into_iter().sum::<u32>());
}
