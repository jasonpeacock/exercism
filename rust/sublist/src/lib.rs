#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.
pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    // This also checks if lists are both empty, it must come before the other empty list checks.
    if first_list.len() == second_list.len() {
        if first_list == second_list {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    }

    if check_list(first_list, second_list) {
        return Comparison::Sublist;
    }

    if check_list(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

/// Superlist is the inverse of Sublist, so we only need to define the checks
/// once, then run it on (A, B) and (B, A) for full coverage.
fn check_list<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.is_empty() {
        return true;
    }

    if second_list.ends_with(first_list) {
        return true;
    }

    for (index, _value) in second_list.iter().enumerate() {
        if index + first_list.len() > second_list.len() {
            break;
        }

        if second_list[index..].starts_with(first_list) {
            return true;
        }
    }

    false
}
