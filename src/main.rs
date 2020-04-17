fn main() {
    let search_txt = String::from("text search gogogogo");
    picking_first_word(&search_txt);
}

fn picking_first_word(s: &String) {
    let word: Vec<_> = s.split(' ').collect();
    println!("{}", word[0]);
}