use std::collections::HashMap;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        a = b;
        b = a % b;
    }
    a
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    let alphamap = HashMap::from_iter(alphabet.chars().enumerate().map(|(idx, c)| (c, idx)));
    let d = gcd(a, b);
    if d > 1 {
        return Err(AffineCipherError::NotCoprime(d));
    }
    let m = alphabet.len() as i32;
    let res = String::with_capacity(plaintext.len());
    for c in plaintext.chars() {
        res.push(alphamap.get(&c));
    }
    Ok(res)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    todo!("Decode {ciphertext} with the key ({a}, {b})");
}
