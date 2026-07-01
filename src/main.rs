pub mod core;

fn main() {
    let test = core::moves::Square::from(29)
        .expect("Invalid input 79")
        .get_human_readable();

    println!("{}{}", test.0, test.1);
}
