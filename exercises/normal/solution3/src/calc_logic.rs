pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑

    if n <= 1 {
        return 0.0;
    }

    let days_in_year: f64 = 365.0;
    let mut probability_no_match = 1.0;

    for i in 0..n {
        probability_no_match *= (days_in_year - i  as f64) / days_in_year;
    }

    1.0 - probability_no_match
}
