const PRODUCTION_MULTIPLIER: f64 = 221.0;

/// Calculate hourly production rate at speed.
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = speed as f64 * PRODUCTION_MULTIPLIER;

    match speed {
        0..=4 => rate,
        5..=8 => rate * 0.90,
        9..=10 => rate * 0.77,
        _ => panic!("Speed is too high: {} > 10", speed),
    }
}

/// Calculate the amount of working items at speed.
pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
