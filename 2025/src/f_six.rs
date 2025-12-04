pub fn solve_six(input: &[u8]) -> u64 {
    // SAFETY: input is valid UTF-8
    let input = unsafe { std::str::from_utf8_unchecked(input) };
    input
        .lines()
        .map(|line| {
            let mut window: [u8; 13] = [0; 13];
            for bytes in line.bytes() {
                window[12] = bytes - b'0';
                for index in 0..12 {
                    if window[index] < window[index + 1] {
                        window.copy_within(index + 1..13, index);
                        break;
                    }
                }
            }
            window[0..12].iter().fold(0u64, |a, &b| 10 * a + b as u64)
        })
        .sum::<u64>()
}
