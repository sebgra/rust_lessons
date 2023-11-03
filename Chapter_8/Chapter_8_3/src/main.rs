use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // define key to access values in HashMap

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);  //copied to get option<i32> instead of option<&i32>, unwrap_or to get 0 instead of None if None return by get 

    println!("{}: {}", team_name, score);

    // iterate over scores

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwrite a value
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Add value if not already present

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
