fn main() {
    let search_txt = String::from("text search gogogogo");
    let word = picking_first_word(&search_txt);
    println!("{}", word);
}

fn picking_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            return &s[..i]
        }
    }
    return &s[..]
}