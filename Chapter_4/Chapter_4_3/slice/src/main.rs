fn main() {

    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("The word is: {word}");
    
    
    // String slices
    
    let hello = &s[0..5];
    let world = &s[6..11];
    
    println!("Word 1 is : {hello}");
    println!("Word 2 is : {world}");

    let slice = &s[0..s.len()]; // Takes the whole string without having the index of the last character

    println!("Slice is: {slice}");

    s.clear(); // empty the string - replace it by ""

    // String litrals

    let mystring_literal = "Hello, world!";

    let word = first_word(&mystring_literal[0..6]);
    let word = first_word(&mystring_literal[..]);

    let word = first_word(mystring_literal);
}

fn first_word(s : &str) -> &str{

    let bytes = s.as_bytes(); // Conversion of string to array of bytes for comparison

    for (i, &item) in bytes.iter().enumerate(){

        if item == b' '{

            return &s[0..i];
        }
    }

    &s[..]
}


