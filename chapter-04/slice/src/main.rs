fn main() {
    let s = String::from("hello world");

    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);

    let word = first_word(&s);

    let s_literal = "hello world";

    let word = first_word(&s_literal[0..6]);
    let word = first_word(&s_literal[..]);

    let word = first_word(&s_literal);

    println!("the first word is: {}", word);
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
