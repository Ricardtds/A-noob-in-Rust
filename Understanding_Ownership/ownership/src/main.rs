fn main() {
    let s = String::from("h ello");
    let b = first_word(&s);
    println!("{} and {}", b, s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

