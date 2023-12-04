use std::fs::read_to_string;

fn is_ascii_punctuation(c: char) -> bool {
    matches!(c, '!'..='-')
        | (c == '/')
        | matches!(c, ':'..='@')
        | matches!(c, '['..='`')
        | matches!(c, '{'..='~')
}

pub fn p1() {
    let input = read_to_string("./inputs/day-3.txt").unwrap();

    let matrix = {
        let lines = input.lines();
        let (_, size) = lines.size_hint();

        let mut matrix = Vec::with_capacity(size.unwrap_or(10));

        for line in lines {
            matrix.push(line.chars().collect::<Vec<char>>());
        }

        matrix
    };

    let mut total = 0;

    for (y, row) in matrix.iter().enumerate() {
        let mut skip_next_n = 0;

        for (x, item) in row.iter().enumerate() {
            if skip_next_n != 0 {
                skip_next_n -= 1;
                continue;
            }

            if item.is_ascii_digit() {
                let next = row.get(x + 1).unwrap_or_else(|| &'a');
                let then = row.get(x + 2).unwrap_or_else(|| &'a');

                let mut last_number_index = x;

                if next.is_ascii_digit() {
                    last_number_index += 1;

                    if then.is_ascii_digit() {
                        last_number_index += 1;
                    }
                }

                let included = 'a: {
                    // Search row above
                    if y != 0 {
                        let row_above = &matrix[y - 1];
                        if x != 0 {
                            let upper_left = row_above[x - 1];
                            if is_ascii_punctuation(upper_left) {
                                break 'a true;
                            }
                        }

                        // Check above and right.
                        for i in x..=last_number_index + 1 {
                            if i + 1 == row.len() {
                                break;
                            }

                            let above = row_above[i];
                            if is_ascii_punctuation(above) {
                                break 'a true;
                            }
                        }
                    }

                    // Search next to
                    if x != 0 {
                        let left = row[x - 1];
                        if is_ascii_punctuation(left) {
                            break 'a true;
                        }
                    }

                    if last_number_index + 2 <= row.len() {
                        let right = row[last_number_index + 1];
                        if is_ascii_punctuation(right) {
                            break 'a true;
                        }
                    }

                    // Search row below
                    if y + 1 != matrix.len() {
                        let row_below = &matrix[y + 1];

                        if x != 0 {
                            let lower_left = row_below[x - 1];

                            if is_ascii_punctuation(lower_left) {
                                break 'a true;
                            }
                        }

                        // Check lower and right.
                        for i in x..=last_number_index + 1 {
                            if i + 1 == row.len() {
                                break;
                            }

                            let lower = row_below[i];
                            if is_ascii_punctuation(lower) {
                                break 'a true;
                            }
                        }
                    }

                    false
                };

                if included {
                    let mut number = String::with_capacity(3);

                    number.push(*item);

                    match last_number_index - x + 1 {
                        2 => number.push(*next),
                        3 => {
                            number.push(*next);
                            number.push(*then)
                        }
                        _ => {}
                    }
                    total += number.parse::<u32>().unwrap();
                }

                skip_next_n += last_number_index - x;
            }
        }
    }

    println!("{total}");
}
