// Traits examples organized in a separate file

use crate::{Difficulty, Example};

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
            commentary: r#"Traits define shared behavior - a set of method signatures that types can implement. They're
similar to interfaces in other languages but more powerful. Any type that implements a trait must
provide implementations for all required methods.

From The Rust Book (Chapter 10.2):
"A trait tells the Rust compiler about functionality a particular type has and can share with
other types."

Traits enable polymorphism and are fundamental to Rust's type system and generics."#,
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
            commentary: r#"Traits can provide default implementations for some or all methods. Types implementing the trait
can choose to use the default or override it with their own implementation. This reduces code
duplication when many types share common behavior.

From The Rust Book (Chapter 10.2):
"Sometimes it's useful to have default behavior for some or all of the methods in a trait instead
of requiring implementations for all methods on every type."

Default methods can call other trait methods, even those without defaults."#,
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
            commentary: r#"Trait bounds constrain generic type parameters to types that implement specific traits. The
syntax T: Printable means "T can be any type that implements Printable." This gives the function
body access to trait methods while maintaining compile-time type safety.

From The Rust Book (Chapter 10.2):
"We can use trait bounds to specify that a generic type can be any type that has certain behavior."

This is how Rust achieves zero-cost abstraction - the compiler generates specialized code for each type."#,
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
            commentary: r#"Multiple trait bounds can be specified using the + syntax. The type parameter must implement
ALL listed traits. This allows functions to use methods from multiple traits, combining different
capabilities like display formatting, comparison, and debugging.

From The Rust Book (Chapter 10.2):
"We can specify multiple trait bounds on a generic type by using the + syntax."

For complex bounds, use where clauses for better readability (shown in next example)."#,
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
            commentary: r#"Trait objects (dyn Trait) enable dynamic dispatch - choosing which method to call at runtime
rather than compile time. This allows collections of different types that share a trait. The
tradeoff is a small runtime cost for the virtual method call.

From The Rust Book (Chapter 17.2):
"The dyn keyword is used before the trait name to indicate we're using trait objects and dynamic
dispatch."

Common gotcha: Not all traits can be trait objects - they must be "object safe" (no generic parameters, etc.)."#,
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
            commentary: r#"The impl Trait syntax allows returning a type that implements a trait without naming the concrete
type. This is useful for returning complex types (like iterators) or hiding implementation details.
Unlike trait objects, this uses static dispatch with zero runtime cost.

From The Rust Book (Chapter 10.2):
"By using impl Trait for the return type, we specify that the function returns some type that
implements the Trait, without naming the concrete type."

Limitation: You can only return a single concrete type - not different types from different branches."#,
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
            commentary: r#"The derive attribute automatically generates trait implementations for common traits. The compiler
creates sensible default implementations based on the struct's fields. This works for Debug, Clone,
Copy, PartialEq, Eq, PartialOrd, Ord, Hash, and Default.

From The Rust Book (Chapter 10.2):
"Rust can provide basic implementations for some traits via the derive attribute."

Common gotcha: derive Clone/Copy requires all fields to also be Clone/Copy. For custom logic, implement manually."#,
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
            commentary: r#"Associated types let you define placeholder types within traits that implementers must specify.
Unlike generic type parameters, a type can only implement a trait with associated types once.
This makes the API cleaner when there's a logical relationship between the trait and the type.

From The Rust Book (Chapter 19.2):
"Associated types connect a type placeholder with a trait such that the trait method definitions
can use these placeholder types in their signatures."

Use associated types when each implementer will have one obvious Item type (like Iterator::Item)."#,
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
            commentary: r#"Operator overloading in Rust is done by implementing traits from std::ops like Add, Sub, Mul, etc.
The Output associated type specifies what type the operation returns. This allows using familiar
operators (+, -, *, /) with custom types while maintaining type safety.

From The Rust Book (Chapter 19.2):
"Rust doesn't allow you to create your own operators or overload arbitrary operators. But you can
overload the operations and corresponding traits listed in std::ops."

Unlike some languages, Rust's operator overloading is explicit and type-checked, preventing surprises."#,
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
            commentary: r#"Supertraits allow one trait to require that implementers also implement another trait. The syntax
trait PrintableShape: Display means any type implementing PrintableShape must also implement Display.
This lets the trait use methods from its supertrait.

From The Rust Book (Chapter 19.2):
"Sometimes, you might need one trait to use another trait's functionality. In this case, you need
to rely on the dependent trait also being implemented."

This is similar to interface inheritance but doesn't inherit implementations, only requirements."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
