fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    change_mut(&mut s1);

    let len = calculate_length(&s1);
    println!("After modification, the length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Does not work because references are immutable by default
//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}