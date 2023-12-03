use std::fs::read_to_string;

pub fn p1() {
    let inputs = read_to_string("./inputs/day-1.txt").unwrap();
    let mut total = 0_u32;

    for line in inputs.lines() {
        let mut first_buffer = [0; 1];
        let mut last_buffer = [0; 1];
        let mut digits = line.chars();

        // SAFETY: Each line will contain at least one digit.
        let first = unsafe { digits.find(|c| c.is_ascii_digit()).unwrap_unchecked() };
        let last = digits.rfind(|c| c.is_ascii_digit()).unwrap_or(first);

        std::char::encode_utf8_raw(first as u32, &mut first_buffer);
        std::char::encode_utf8_raw(last as u32, &mut last_buffer);

        // SAFETY: Src and Dst have the same size.
        let combined_buffer = unsafe { std::mem::transmute::<_, [u8; 2]>([first_buffer, last_buffer]) };

        // SAFETY: Encoded charactars are guaranteed to be valid.
        let ret = unsafe {
            std::str::from_utf8_unchecked(&combined_buffer)
                .parse::<u32>()
                .unwrap_unchecked()
        };
        total += ret;
    }

    println!("{total}");
}

pub fn p2() {
    let inputs = read_to_string("./inputs/day-1.txt").unwrap();

    let total = inputs
        .lines()
        .map(|l| {
            let s = l
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "th3ee")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "se7n")
                .replace("eight", "e8ht")
                .replace("nine", "n9e");

            let mut first_buffer = [0; 1];
            let mut last_buffer = [0; 1];
            let mut digits = s.chars();

            // SAFETY: Each line will contain at least one digit.
            let first = unsafe { digits.find(|c| c.is_ascii_digit()).unwrap_unchecked() };
            let last = digits.rfind(|c| c.is_ascii_digit()).unwrap_or(first);

            std::char::encode_utf8_raw(first as u32, &mut first_buffer);
            std::char::encode_utf8_raw(last as u32, &mut last_buffer);

            let combined_buffer = unsafe { std::mem::transmute::<_, [u8; 2]>([first_buffer, last_buffer]) };

            // SAFETY: Encoded charactars are guaranteed to be valid.
            unsafe {
                std::str::from_utf8_unchecked(&combined_buffer)
                    .parse::<u32>()
                    .unwrap_unchecked()
            }
        })
        .sum::<u32>();

    println!("{total}")
}
