// Traits: Defining shared behavior

use crate::{Difficulty, Example};

// Example traits and types for demonstration
pub trait Summary {
    fn summarize(&self) -> String;

    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Article {
    pub headline: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "basic_trait",
            description: "Defining and implementing a basic trait",
            code: r#"trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.name)
    }
}

let person = Person { name: "Alice".to_string() };
println!("{}", person.greet());"#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "default_implementation",
            description: "Traits with default method implementations",
            code: r#"trait Speak {
    fn speak(&self) -> String {
        String::from("...")
    }

    fn introduce(&self) -> String;
}

struct Dog;

impl Speak for Dog {
    fn introduce(&self) -> String {
        String::from("I'm a dog")
    }
}

let dog = Dog;
println!("{}", dog.introduce());
println!("{}", dog.speak());  // Uses default"#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "trait_bounds",
            description: "Using traits as bounds in generic functions",
            code: r#"trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("Number: {}", self)
    }
}

fn print_item<T: Printable>(item: T) {
    println!("{}", item.format());
}

print_item(42);"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "multiple_trait_bounds",
            description: "Multiple trait bounds with + syntax",
            code: r#"use std::fmt::{Display, Debug};

fn compare_display<T: Display + Debug + PartialOrd>(a: T, b: T) {
    println!("Comparing {:?} and {:?}", a, b);
    if a > b {
        println!("{} is greater", a);
    } else {
        println!("{} is greater", b);
    }
}

compare_display(10, 20);"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "trait_objects",
            description: "Using trait objects for dynamic dispatch",
            code: r#"trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof!")
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("Meow!")
    }
}

let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];

for animal in animals {
    println!("{}", animal.make_sound());
}"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "return_trait",
            description: "Returning types that implement traits",
            code: r#"trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

fn create_shape() -> impl Shape {
    Circle { radius: 5.0 }
}

let shape = create_shape();
println!("Area: {}", shape.area());"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "derive_traits",
            description: "Automatically deriving common traits",
            code: r#"#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1.clone();

println!("p1: {:?}", p1);
println!("p2: {:?}", p2);
println!("Equal? {}", p1 == p2);"#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "associated_types",
            description: "Traits with associated types",
            code: r#"trait Container {
    type Item;

    fn add(&mut self, item: Self::Item);
    fn get(&self) -> Option<&Self::Item>;
}

struct NumberContainer {
    value: Option<i32>,
}

impl Container for NumberContainer {
    type Item = i32;

    fn add(&mut self, item: i32) {
        self.value = Some(item);
    }

    fn get(&self) -> Option<&i32> {
        self.value.as_ref()
    }
}

let mut container = NumberContainer { value: None };
container.add(42);
if let Some(val) = container.get() {
    println!("Value: {}", val);
}"#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "operator_overloading",
            description: "Overloading operators with traits",
            code: r#"use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };
let p3 = p1 + p2;

println!("p1 + p2 = {:?}", p3);"#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "supertraits",
            description: "Traits that depend on other traits",
            code: r#"use std::fmt::Display;

trait PrintableShape: Display {
    fn area(&self) -> f64;

    fn describe(&self) {
        println!("{} has area {:.2}", self, self.area());
    }
}

struct Circle {
    radius: f64,
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Circle(r={})", self.radius)
    }
}

impl PrintableShape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

let circle = Circle { radius: 5.0 };
circle.describe();"#,
            difficulty: Difficulty::Advanced,
        },
    ]
}

pub fn run_example(name: &str) {
    match name {
        "basic_trait" => basic_trait(),
        "default_implementation" => default_implementation(),
        "trait_bounds" => trait_bounds(),
        "multiple_trait_bounds" => multiple_trait_bounds(),
        "trait_objects" => trait_objects(),
        "return_trait" => return_trait(),
        "derive_traits" => derive_traits(),
        "associated_types" => associated_types(),
        "operator_overloading" => operator_overloading(),
        "supertraits" => supertraits(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_trait() {
    trait Greet {
        fn greet(&self) -> String;
    }

    struct Person {
        name: String,
    }

    impl Greet for Person {
        fn greet(&self) -> String {
            format!("Hello, I'm {}", self.name)
        }
    }

    let person = Person {
        name: "Alice".to_string(),
    };
    println!("{}", person.greet());
}

fn default_implementation() {
    trait Speak {
        fn speak(&self) -> String {
            String::from("...")
        }

        fn introduce(&self) -> String;
    }

    struct Dog;

    impl Speak for Dog {
        fn introduce(&self) -> String {
            String::from("I'm a dog")
        }
    }

    let dog = Dog;
    println!("{}", dog.introduce());
    println!("{}", dog.speak());
}

fn trait_bounds() {
    trait Printable {
        fn format(&self) -> String;
    }

    impl Printable for i32 {
        fn format(&self) -> String {
            format!("Number: {}", self)
        }
    }

    fn print_item<T: Printable>(item: T) {
        println!("{}", item.format());
    }

    print_item(42);
}

fn multiple_trait_bounds() {
    use std::fmt::{Debug, Display};

    fn compare_display<T: Display + Debug + PartialOrd>(a: T, b: T) {
        println!("Comparing {:?} and {:?}", a, b);
        if a > b {
            println!("{} is greater", a);
        } else {
            println!("{} is greater", b);
        }
    }

    compare_display(10, 20);
}

fn trait_objects() {
    trait Animal {
        fn make_sound(&self) -> String;
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn make_sound(&self) -> String {
            String::from("Woof!")
        }
    }

    impl Animal for Cat {
        fn make_sound(&self) -> String {
            String::from("Meow!")
        }
    }

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    for animal in animals {
        println!("{}", animal.make_sound());
    }
}

fn return_trait() {
    trait Shape {
        fn area(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            3.14159 * self.radius * self.radius
        }
    }

    fn create_shape() -> impl Shape {
        Circle { radius: 5.0 }
    }

    let shape = create_shape();
    println!("Area: {}", shape.area());
}

fn derive_traits() {
    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("Equal? {}", p1 == p2);
}

fn associated_types() {
    trait Container {
        type Item;

        fn add(&mut self, item: Self::Item);
        fn get(&self) -> Option<&Self::Item>;
    }

    struct NumberContainer {
        value: Option<i32>,
    }

    impl Container for NumberContainer {
        type Item = i32;

        fn add(&mut self, item: i32) {
            self.value = Some(item);
        }

        fn get(&self) -> Option<&i32> {
            self.value.as_ref()
        }
    }

    let mut container = NumberContainer { value: None };
    container.add(42);
    if let Some(val) = container.get() {
        println!("Value: {}", val);
    }
}

fn operator_overloading() {
    use std::ops::Add;

    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;

    println!("p1 + p2 = {:?}", p3);
}

fn supertraits() {
    use std::fmt::Display;

    trait PrintableShape: Display {
        fn area(&self) -> f64;

        fn describe(&self) {
            println!("{} has area {:.2}", self, self.area());
        }
    }

    struct Circle {
        radius: f64,
    }

    impl Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Circle(r={})", self.radius)
        }
    }

    impl PrintableShape for Circle {
        fn area(&self) -> f64 {
            3.14159 * self.radius * self.radius
        }
    }

    let circle = Circle { radius: 5.0 };
    circle.describe();
}
