mod one_layer {
    pub fn print_characters() {
        for ch in 'a'..='Z' {
            println!("{}", ch);
        }
    }
}

mod two_layer {
    pub fn print_characters() {
        for ch in 'A'..='z' {
            println!("{}", ch);
        }
    }
}

fn main() {
    println!("One layer characters:");
    one_layer::print_characters();

    println!("Two layer characters:");
    two_layer::print_characters();
}

