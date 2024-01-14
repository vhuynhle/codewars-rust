pub fn round_up_to_5(n: i32) -> i32 {
    // If n >= 0, n mod 5 in [0..=4]
    // If n < 0, n mod 5 in [-4..0]
    // In both cases, m = (5 - (n % 5)) % 5 in [0..=4] and (n + m) mod 5 = 0.
    n + (5 - (n % 5)) % 5
}
