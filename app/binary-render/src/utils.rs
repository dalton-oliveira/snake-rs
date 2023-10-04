pub fn to_base_10_array(n: u16, digits: u8) -> Vec<u8> {
    let mut digits = vec![0; digits as usize];
    let mut n = n;
    let mut i = digits.len() - 1;
    while n > 0 {
        digits[i] = (n % 10) as u8;
        i -= 1;
        n /= 10;
    }

    return digits;
}
