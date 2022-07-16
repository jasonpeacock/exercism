const EXPECTED_MINUTES_IN_OVEN: i32 = 40;
const MINUTES_PER_LAYER: i32 = 2;

/// Return expected minutes in the oven.
pub fn expected_minutes_in_oven() -> i32 {
    EXPECTED_MINUTES_IN_OVEN
}

/// Calculate remaining minutes in oven given actual minutes in oven.
pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

/// Calculate preparation time in minutes for number of layers.
pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * MINUTES_PER_LAYER
}

/// Calculate elapsed time in minutes for number of layers and actual minutes in oven.
pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
