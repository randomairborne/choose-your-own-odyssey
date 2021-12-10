#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod choose_your_own_adventure;

fn main() {
    println!("You reach the island, and notice the many sheep wandering.");
    let choice = choose_your_own_adventure::choose(
        "Where do you go from here?",
        vec!["The hill".to_string(), "The meadow".to_string()],
    );
    if choice == *"The hill" {
        println!("went to hill");
    } else if choice == *"The meadow" {
        println!("went to meadow");
    }
}
