//use crate::print_charactersatoZ;
use crate::one_layer::print_charactersatoZ;
use crate::two_layer::print_charactersAtoz;

mod one_layer;
mod two_layer;

fn main() {
    println!("One layer characters:");
    print_charactersatoZ();

    println!("Two layer characters:");
    print_charactersAtoz();
}
