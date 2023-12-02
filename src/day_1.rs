use std::fs::read_to_string;

pub fn run() {
    let inputs = read_to_string("./inputs/day-1.txt").unwrap();
    let mut total = 0_u32;

    for line in inputs.lines() {
        let mut first_buffer = [0; 1];
        let mut last_buffer = [0; 1];
        let mut digits = line.chars();

        // SAFETY: Each line will contain at least one digit.
        let first = unsafe { digits.find(|c| c.is_ascii_digit()).unwrap_unchecked() };
        let last = digits.rfind(|c| c.is_ascii_digit()).unwrap_or(first);

        let f = std::char::encode_utf8_raw(first as u32, &mut first_buffer);
        let l = std::char::encode_utf8_raw(last as u32, &mut last_buffer);

        let mut s: Vec<u8> = Vec::with_capacity(2);
        s.extend_from_slice(f);
        s.extend_from_slice(l);

        // SAFETY: Encoded charactars are guaranteed to be valid.
        let ret = unsafe {
            String::from_utf8_unchecked(s)
                .parse::<u32>()
                .unwrap_unchecked()
        };
        total += ret;
    }

    println!("{total}");
}
