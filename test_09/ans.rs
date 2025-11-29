pub fn apply_pipeline<T, U, F>(input: Vec<T>, mut f: F) -> Vec<U>
where
    F: FnMut(T) -> Option<U>,
{
    let mut output = Vec::new();
    for item in input.into_iter() {
        if let Some(value) = f(item) {
            output.push(value);
        }
    }
    output
}
