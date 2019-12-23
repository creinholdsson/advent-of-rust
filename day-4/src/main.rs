fn is_valid_password(password: &str) -> bool {
    let mut last_char = '0';
    let mut found_double = false;
    for chr in password.chars() {
        if !(chr >= '0' && chr <= '9' && chr >= last_char) {
            return false;
        }
        if chr == last_char {
            found_double = true;
        }

        last_char = chr;
    }
    found_double
}

fn count_valid_passwords(lower_limit: i32, upper_limit: i32) -> i32 {
    (lower_limit..=upper_limit)
        .filter(|&x| is_valid_password(&*x.to_string()))
        .count() as i32
}

fn main() {
    println!(
        "Valid passwords: {}",
        count_valid_passwords(156_218, 652_527)
    );
}

#[test]
fn test_is_valid_password() {
    assert_eq!(true, is_valid_password("111111"));
    assert_eq!(false, is_valid_password("223450"));
    assert_eq!(false, is_valid_password("123789"));
}
