
pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return  String::new();
    }
    let mut output = String::new();
    let mut right =  (input.chars().count() - 1) as i64;
    while right >= 0 {
        output.push(input.chars().nth(right as usize).unwrap());
        right -= 1
    }

    output
}
