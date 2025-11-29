pub fn average_even(numbers: &[i32]) -> Option<f64> {
    let mut sum = 0i64;
    let mut count = 0u32;
    for &n in numbers {
        if n % 2 == 0 {
            sum += n as i64;
            count += 1;
        }
    }
    if count == 0 {
        None
    } else {
        Some(sum as f64 / count as f64)
    }
}
