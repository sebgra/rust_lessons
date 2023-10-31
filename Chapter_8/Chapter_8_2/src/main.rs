fn main() {
    let mut s = String::new();

        let data = "Initial contents";

        let mut s = data.to_string();

        println!("{}", data);

        // Other possibility

        // let s = "Initial contents".to_string();


        s = String::from("Post initial content");

        println!("{}", s);

        // Adding content

        s.push_str(" with new content");

        println!("{}", s);

        let mut s1 = String::from("Hello, ");
        let mut s2 = String::from("world!");
        let mut s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

        // Need to use reference as second parameter

        println!("Content of s3 : {s3}");

        // Multiple concatenations

        s1 = String::from("tic");
        s2 = String::from("tac");
        s3 = String::from("toe");

        s = s1 + "-" + &s2 + "-" + &s3; // Multiple need of using references such as &s2 and &s3
        println!("Content of s : {s}");
        
        // Iterate over strings

        for c in s.chars(){

            println!("Current character of s : {c}");
        }

        for b in s.bytes(){

            println!("Current byte of s : {b}");
        }
    


}
