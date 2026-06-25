pub fn encode(inner_source: &str) -> String {
    if inner_source.is_empty() {
        return String::new();
    }
    let mut current = inner_source.chars().next().unwrap();
    let mut cursor = 0;
    let mut buf = vec![];
    while cursor < inner_source.chars().count() {
        let mut j = cursor;
        while j < inner_source.chars().count() && inner_source.chars().nth(j).unwrap() == current {
            j += 1;
        }
        if j - cursor > 1 {
            buf.push(format!("{}", j - cursor)); // length
            buf.push(format!("{}", current)); // char
        } else {
            buf.push(format!("{}", current)); // char
        }
        if j == inner_source.chars().count() {
            break;
        }
        current = inner_source.chars().nth(j).unwrap();
        cursor = j;
    }
    buf.join("")
}

#[test]
fn test_encode() {
    assert_eq!(encode("AAAAABBC"), String::from("5A2BC"));
    assert_eq!(encode("AAAABBC"), String::from("4A2BC"));

    assert_eq!(encode("AAAAAAAAAABBC"), String::from("10A2BC"));
    assert_eq!(
        encode("AAAAAAAAAABBCDDDDDDDDDDDD"),
        String::from("10A2BC12D")
    );

    assert_eq!(encode("ABC"), String::from("ABC"));
    assert_eq!(encode(""), String::from(""));

    let input = "  hsqq qww  ";
    let output = encode(input);
    let expected = "2 hs2q q2w2 ";
    assert_eq!(output, expected);
}

pub fn decode(source: &str) -> String {
    let mut cur = 0;
    let length = source.chars().count();
    let mut result = vec![];
    while cur < length {
        // length
        let mut j = cur;
        while j < length && source.chars().nth(j).unwrap().is_numeric() {
            j += 1;
        }
        // here the cur does not have numeric
        let char = source.chars().nth(j).unwrap().to_string();
        if j - cur > 0 {
            let charlen = source.chars().skip(cur).take(j - cur).collect::<String>();
            let clen = charlen.parse::<usize>().unwrap();
            result.extend(std::iter::repeat_n(char, clen));
        } else {
            result.push(char);
        }
        cur = j + 1;
    }
    result.join("")
}

#[test]
fn test_decode() {
    let input = "12WB12W3B24WB";
    let output = decode(input);
    let expected = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";
    assert_eq!(output, expected);
}

#[test]
fn decode_edge() {
    let input = "12WB12W3B24WB";
    let output = decode(input);
    let expected = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";
    assert_eq!(output, expected);
}
