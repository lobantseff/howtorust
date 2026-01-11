// Ownership example runner functions organized in a separate file

pub fn run_example(name: &str) {
    match name {
        "basic_ownership" => basic_ownership(),
        "clone_vs_move" => clone_vs_move(),
        "function_ownership" => function_ownership(),
        "borrowing_immutable" => borrowing_immutable(),
        "borrowing_mutable" => borrowing_mutable(),
        "multiple_references" => multiple_references(),
        "reference_rules" => reference_rules(),
        "slice_internals" => slice_internals(),
        "dangling_reference_prevention" => dangling_reference_prevention(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2: {}", s2);
}

fn clone_vs_move() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
}

fn function_ownership() {
    fn takes_ownership(s: String) {
        println!("Inside function: {}", s);
    }

    let s = String::from("hello");
    takes_ownership(s);
}

fn borrowing_immutable() {
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{}' has length {}", s1, len);
}

fn borrowing_mutable() {
    fn append_world(s: &mut String) {
        s.push_str(", world!");
    }

    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s);
}

fn multiple_references() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{}, {}, {}", r1, r2, r3);
}

fn reference_rules() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}

fn slice_internals() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("First word: {}, second word: {}", hello, world);
    println!("Original string still valid: {}", s);
}

fn dangling_reference_prevention() {
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }

    let s = no_dangle();
    println!("{}", s);
}
