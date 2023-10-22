// one_layer.rs


pub fn print_charactersatoZ() {
    for ch1 in 'a'..='z' {
        if ch1.is_ascii_alphabetic() {
            println!("{}", ch1);
        }
    }

    for ch2 in 'A'..='Z' {
        if ch2.is_ascii_alphabetic() {
            println!("{}", ch2);
        }
    }
}
