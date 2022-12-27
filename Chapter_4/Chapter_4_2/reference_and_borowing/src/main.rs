fn main() {

    let mut s = String::from("hello!");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Hello, world!");

    // Mutable references
    let mut s2 = String::from("hello");

    // Using scope to allow multiple mutable references

    {
        let r1 = &mut s;
    }

    let r2 = &mut s;
}

fn calculate_length(s: &String) -> usize{

    s.len()
}

fn change(some_string : &mut String){

    some_string.push_str(", world!");
}