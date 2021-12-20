#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod choose_your_own_adventure;

use choose_your_own_adventure::choose;

fn main() {
    println!("You reach the island, and notice the many sheep wandering.");
    wait!(200);
    println!("Do you go to the mountain to talk to the cyclops or try to steal the sheep?");
    wait!(200);
    let choice = choose(
        vec!["Go to the mountain".to_string(), "Steal the sheep".to_string()],
    );
    if choice == *"Go to the mountain" {
        println!("You go to the mountain and go into the cyclops' cave, waiting for him to return.");
        wait!(200);
        println!("He returns, driving his sheep and goats, in to milk them");
        wait!(200);
        println!("What do you do next?");
        let choice = choose(
            vec!["Attack the cyclops".to_string(), "Ask to sit and eat with him".to_string(), "Try to run past the rock before he closes it".to_string()],
        );
        if choice == *"Attack the cyclops" {
            println!("Not finished:tm:");
        } else if choice == *"Ask to sit and eat with him" {
            println!("The cyclops asks who you are and what your business is.");
            wait!(200);
            println!("How do you respond?");
            let choice = choose( vec!["Travelers, and ask him to show hospitality".to_string(), "Warriors, and challenge him to fight".to_string()]);
            if choice == *"Travelers, and ask him to show hospitality" {

            } else if choice == *"Warriors, and challenge him to fight" {
                println!("As the cyclops charges you, you stab at his ankles, but he crushes you underfoot and you are destroyed.");
            }
        } else if choice == *"Try to run past the rock before he closes it" {
            println!("Not finished:tm:");
        }
    } else if choice == *"Steal the sheep" {
        println!("Not implemented yet:tm:");
    }
}
