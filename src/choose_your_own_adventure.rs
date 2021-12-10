pub fn wait(millis: u64) {
    // Sleep for this many milliseconds
    std::thread::sleep(std::time::Duration::from_millis(millis));
}

pub fn choose(prompt: &str, options: Vec<String>) -> String {
    loop {
        let mut itr = 0;
        let mut choices: std::collections::HashMap<i8, String> = std::collections::HashMap::new();
        for option in &options {
            itr += 1;
            wait(200);
            println!("{}: {}", itr, option.to_string());
            choices.insert(itr, option.to_string());
        }
        wait(200);
        println!("{}", prompt);
        let number_picked: i8 = text_io::read!("{}\n");
        match choices.get(&number_picked) {
            Some(review) => return review.to_string(),
            None => println!("Invalid input, please try again"),
        }
    }
}
