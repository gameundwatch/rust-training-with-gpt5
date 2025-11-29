pub fn rectangle_report(width: u32, height: u32) -> (u32, u32, bool) {
    let area = width * height;
    let perimeter = 2 * (width + height);
    let is_square = width == height;
    return (area, perimeter, is_square);
}
