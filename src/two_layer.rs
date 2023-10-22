pub fn print_charactersAtoz() {
    for ch in 'A'..='z' {
        if ch.is_ascii_alphabetic() {
            println!("{}", ch);
        }
    }
}
