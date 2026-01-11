// Generics examples organized in a separate file

use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "generic_function",
            description: "Basic generic function",
            code: r#"fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

let numbers = vec![34, 50, 25, 100, 65];
println!("Largest number: {}", largest(&numbers));

let chars = vec!['y', 'm', 'a', 'q'];
println!("Largest char: {}", largest(&chars));"#,
            commentary: r#"Generic functions work with multiple types using type parameters (T). The PartialOrd trait bound
ensures T can be compared. The compiler generates a separate version of this function for each
concrete type used (monomorphization), resulting in zero runtime overhead.

From The Rust Book (Chapter 10.1):
"Generics allow us to replace specific types with a placeholder that represents multiple types."

This is more powerful than runtime polymorphism as it maintains type safety with no performance cost."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "generic_struct",
            description: "Generic struct with single type parameter",
            code: r#"#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

let integer_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.0, y: 4.0 };

println!("Integer point: {:?}", integer_point);
println!("Float point: {:?}", float_point);"#,
            commentary: r#"Generic structs can hold values of any type using type parameters. Point<T> means both x and y
must be the same type T. When you create an instance, Rust infers the concrete type from the values
you provide. This enables writing reusable data structures.

From The Rust Book (Chapter 10.1):
"We can define structs to use a generic type parameter in one or more fields."

Common gotcha: Point<T> requires both fields be the same type. Use Point<T, U> for different types."#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "multiple_generic_params",
            description: "Generic struct with multiple type parameters",
            code: r#"#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

let pair1 = Pair { first: 5, second: "hello" };
let pair2 = Pair { first: 1.5, second: 'c' };

println!("Pair 1: {:?}", pair1);
println!("Pair 2: {:?}", pair2);"#,
            commentary: r#"Structs can have multiple independent type parameters. Pair<T, U> allows first and second to be
different types. This is more flexible than using a single type parameter and enables heterogeneous
pairs, tuples, and other data structures.

From The Rust Book (Chapter 10.1):
"Using too many type parameters is a sign that your code could benefit from restructuring into
smaller pieces."

Multiple type parameters are common in collections like HashMap<K, V> and Result<T, E>."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "generic_methods",
            description: "Implementing methods on generic types",
            code: r#"struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

let p = Point { x: 3.0_f32, y: 4.0_f32 };
println!("x coordinate: {}", p.x());
println!("Distance: {}", p.distance_from_origin());"#,
            commentary: r#"You can implement methods on generic types in two ways: impl<T> works for any T, while
impl Point<f32> only works for a specific concrete type. This allows generic methods for all types
plus specialized methods for specific types where needed.

From The Rust Book (Chapter 10.1):
"We can implement methods on the struct that use the generic type parameter, or we can implement
methods only for specific concrete types."

The impl<T> syntax declares T as a generic parameter for the impl block."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "generic_enums",
            description: "Generic enums like Option and Result",
            code: r#"enum MyOption<T> {
    Some(T),
    None,
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

let some_number = MyOption::Some(5);
let some_string = MyOption::Some("hello");
let no_value: MyOption<i32> = MyOption::None;

let success: MyResult<i32, String> = MyResult::Ok(200);
let failure: MyResult<i32, String> = MyResult::Err("Error!".to_string());

println!("Created generic enum instances");"#,
            commentary: r#"Generic enums can hold values of different types in their variants. Option<T> and Result<T, E>
are standard library examples - Option handles nullable values, and Result handles success/error cases.
Each enum variant can contain different generic types.

From The Rust Book (Chapter 10.1):
"Enums can use multiple generic types as well. The definition of the Result enum is one example."

Generic enums are central to Rust's error handling and enable type-safe pattern matching."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "trait_bounds",
            description: "Using trait bounds to constrain generics",
            code: r#"fn print_debug<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

fn compare<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}

print_debug(42);
print_debug("hello");
println!("5 > 3: {}", compare(5, 3));
println!("'a' > 'z': {}", compare('a', 'z'));"#,
            commentary: r#"Trait bounds constrain what types can be used with generics. T: Debug means T must implement Debug,
giving the function body access to debug formatting. Without bounds, the compiler doesn't know what
operations are valid on T.

From The Rust Book (Chapter 10.2):
"When defining a function that uses generics, we place trait bounds in the signature of the function
where we would usually specify the data types of the parameters and return value."

Think of trait bounds as capabilities that the type must provide."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "where_clause",
            description: "Using where clauses for complex bounds",
            code: r#"fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    println!("t: {}", t);
    println!("u: {:?}", u);
    42
}

let result = some_function(&"hello", &vec![1, 2, 3]);
println!("Function returned: {}", result);"#,
            commentary: r#"Where clauses provide a cleaner syntax for complex trait bounds. Instead of cluttering the function
signature with T: Trait1 + Trait2, you can list bounds after the where keyword. This is especially
useful with multiple type parameters and complex bounds.

From The Rust Book (Chapter 10.2):
"Specifying trait bounds with a where clause is clearer when specifying many generic types and
trait bounds."

Where clauses also enable expressing more complex relationships between generic types."#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "generic_associated_types",
            description: "Using generic methods with different type parameters",
            code: r#"struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn mixup<U>(self, other: Point<U>) -> Point<(T, U)> {
        Point {
            x: (self.x, other.x),
            y: (self.y, other.y),
        }
    }
}

let p1 = Point { x: 5, y: 10 };
let p2 = Point { x: "Hello", y: "World" };
let p3 = p1.mixup(p2);

println!("p3.x: ({}, {})", (p3.x).0, (p3.x).1);
println!("p3.y: ({}, {})", (p3.y).0, (p3.y).1);"#,
            commentary: r#"Methods on generic types can introduce their own generic parameters. The mixup method takes a
Point<T> (self) and Point<U> (parameter) and returns Point<(T, U)>. This shows how generic parameters
can be mixed and transformed in sophisticated ways.

From The Rust Book (Chapter 10.1):
"Methods can take generic parameters that are different from the ones defined on the struct."

This enables powerful composition - combining different generic types in method calls."#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "const_generics",
            description: "Const generics for compile-time values",
            code: r#"struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Default + Copy, const N: usize> ArrayPair<T, N> {
    fn new() -> Self {
        ArrayPair {
            left: [T::default(); N],
            right: [T::default(); N],
        }
    }
}

let pair: ArrayPair<i32, 3> = ArrayPair::new();
println!("Created array pair with size 3");

let pair2: ArrayPair<f64, 5> = ArrayPair::new();
println!("Created array pair with size 5");"#,
            commentary: r#"Const generics allow using compile-time constant values as generic parameters. The const N: usize
syntax creates a generic parameter for the array size. This enables writing array-based code that
works with any size at compile time.

From The Rust Book (Chapter 10.1 - newer editions):
"Const generics allow types to be generic over constant values."

Before const generics, you couldn't write generic code over array sizes - you'd need separate
implementations for [T; 3], [T; 5], etc. This feature was stabilized in Rust 1.51."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
