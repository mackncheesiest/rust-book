fn main() {
    let s = String::from("hello there this has six words");
    let index_that_ends_first_word = first_word_end_idx(&s);
    println!("Index: {}", index_that_ends_first_word);
    // This sucks b.c. this index is completely separated from the string
    // So you could potentially access using this index later on after the string has been cleared

    let first_word_slice = first_word(&s[..]);
    //s.clear(); // Note: s must be mutable in order to be cleared
    println!("First word: {}", first_word_slice);

    let first_word_slice2 = first_word("this is a string literal that has a string slice type");
    println!("First word: {}", first_word_slice2);

    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..4];
    for (_, &item) in arr_slice.iter().enumerate() {
        println!("element: {}", item);
    }
}

fn first_word_end_idx(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}