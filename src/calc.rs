fn sum(a: i16, b: i16) -> Option<i16> {
    a.checked_add(b)
}

fn sub(a: i16, b: i16) -> Option<i16> {
    a.checked_sub(b)
}

fn div(a: i16, b: i16) -> Option<i16> {
    a.checked_div(b)
}
fn mul(a: i16, b: i16) -> Option<i16> {
    a.checked_mul(b)
}

pub(crate) fn process_equation(equation: &str) -> Option<i16> {
    Some(0)
}
