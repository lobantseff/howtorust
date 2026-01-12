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
            commentary: r#"Generic functions use type parameters to work with multiple types while maintaining type safety. The <T: PartialOrd> syntax declares T as a type parameter with the PartialOrd trait bound, ensuring T supports comparison operations like >. When you call largest with different types, the compiler generates specialized versions of the function for each concrete type through a process called monomorphization. This means largest::<i32> and largest::<char> are literally different functions in the compiled binary, each optimized for its specific type.

Monomorphization happens at compile time, transforming generic code into specialized code for each type actually used. The compiler analyzes all call sites, determines which concrete types are needed, and generates optimized machine code for each. This is why Rust generics have zero runtime overhead - there's no dynamic dispatch, no type checking at runtime, no indirection. The code runs exactly as fast as if you'd written separate functions for each type manually. However, this can increase binary size if many different types are used with the same generic function.

The trait bound T: PartialOrd is essential - without it, the compiler doesn't know what operations are valid on T. Generic type parameters start with no capabilities; trait bounds add them. This is fundamentally different from object-oriented inheritance where types start with capabilities and optionally override them. Rust's approach enables powerful optimizations because the compiler knows exactly what operations exist and can inline and specialize them aggressively.

In practice, generics enable writing highly reusable code without sacrificing performance. The standard library is full of generic functions (sort, map, filter) that work with any type meeting their trait bounds. When designing generic APIs, choose minimal trait bounds that express exactly what capabilities you need. This makes your functions maximally reusable while still type-safe. The compiler's error messages about missing trait bounds guide you toward the right constraints."#,
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
            commentary: r#"Generic structs can hold values of any type using type parameters, enabling reusable data structures that work with multiple types. Point<T> declares that x and y must be the same type T - this constraint is enforced by the struct definition itself. When you create Point { x: 5, y: 10 }, the compiler infers T = i32 from the integer literals. For Point { x: 1.0, y: 4.0 }, it infers T = f64. Type inference makes generic structs convenient to use while maintaining full type safety and compile-time checking.

The single type parameter T creates a constraint: both fields must have identical types. This is often what you want for mathematical concepts like points or coordinates where mixing types would be semantically wrong. If you tried Point { x: 5, y: 4.0 }, the compiler would error because it can't unify integer and float types. This compile-time checking prevents type errors that would be runtime bugs in dynamically typed languages. The struct definition acts as a contract about what types can be combined.

Generic structs are monomorphized just like generic functions - Point<i32> and Point<f64> are distinct types in the compiled binary with no runtime overhead. The compiler generates specialized versions with concrete types, enabling aggressive optimization. Memory layout is determined at compile time, and field access is as fast as non-generic structs. There's no boxing, no indirection, no type tags - just efficient machine code.

In practice, generic structs are fundamental to Rust's standard library. Vec<T>, HashMap<K, V>, Option<T>, and Result<T, E> are all generic structs. When designing your own, consider whether fields should share type parameters or have independent ones. Single type parameters like Point<T> express "these things must be the same type," while multiple parameters like Pair<T, U> allow heterogeneity. Choose based on your domain requirements and type relationships."#,
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
            commentary: r#"Structs with multiple type parameters allow different fields to have independent types, enabling heterogeneous data structures. Pair<T, U> declares that first and second can be completely unrelated types - T and U are independent type variables that don't constrain each other. This flexibility is essential for data structures that naturally combine different types, like key-value pairs, tagged unions, or results that pair success and error types. Each type parameter is separately monomorphized, creating specialized versions for each unique combination of concrete types used.

Multiple type parameters multiply the flexibility of your generic code. Pair<T, U> can represent any combination: Pair<i32, String>, Pair<char, Vec<f64>>, or even Pair<Point<i32>, HashMap<String, bool>>. The compiler generates separate specialized versions for each unique combination actually used in your program. This compile-time specialization means there's no runtime overhead despite the flexibility - each instance has a fixed, known type with deterministic memory layout and optimal machine code.

While multiple type parameters increase expressiveness, they also increase complexity. As the Rust book warns, too many type parameters can make code hard to understand and use. Each parameter adds cognitive load for users who must understand what types to provide and how they relate. Consider whether your struct would be clearer as multiple simpler types or whether associated types might express relationships better. The standard library shows good examples: HashMap<K, V> needs two parameters because keys and values are fundamentally independent, while Iterator uses an associated type because each iterator has one natural item type.

In practice, common patterns include HashMap<K, V> for key-value stores, Result<T, E> for operations with success and error types, and Pair/Tuple types for combining heterogeneous data. When designing APIs with multiple type parameters, provide reasonable defaults or helper functions to reduce verbosity. Consider whether all combinations of types make semantic sense - sometimes trait bounds on one parameter depending on another express important relationships and constraints."#,
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
            commentary: r#"Methods on generic types can be implemented generically for all possible type parameters, or specialized for specific concrete types. The impl<T> Point<T> block declares methods available for Point with any type T, while impl Point<f32> declares methods only for Point<f32>. This dual approach lets you provide universal methods that work with any type parameter plus specialized methods that leverage specific type capabilities. The x() method works for all Points, while distance_from_origin() only makes sense for Point<f32> since it needs floating-point arithmetic.

The impl<T> syntax is crucial - it declares T as a type parameter for the entire impl block, distinct from the struct's type parameter. Without the <T>, you'd be implementing methods for a concrete type named T (which doesn't exist). With <T>, you're saying "for any type T, implement these methods on Point<T>." The compiler generates separate implementations for each monomorphized version, so Point<i32>::x() and Point<f64>::x() are different functions in the compiled binary, each optimized for its specific type.

Specialized implementations enable type-specific optimizations and operations. You might implement fast bitwise methods for Point<u32>, geometric calculations for Point<f64>, or string formatting for Point<char>. Each specialized impl block can have different methods with different signatures. This is more flexible than inheritance - you're not overriding inherited methods, you're adding entirely new capabilities to specific instantiations. The type system ensures you can't accidentally call specialized methods on wrong types.

In practice, start with generic implementations for broadly applicable methods, then add specialized implementations only when specific types truly need different behavior or have unique capabilities. Don't over-specialize - it increases maintenance burden since changes may need to be replicated across multiple impl blocks. Common pattern: generic methods for accessing and comparing data, specialized methods for type-specific operations like mathematical calculations on numeric types or iteration over collection types."#,
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
            commentary: r#"Generic enums allow variants to hold values of parameterized types, enabling type-safe tagged unions that work with any type. MyOption<T> is a reimplementation of Rust's Option - Some(T) holds a value of type T, while None holds nothing. MyResult<T, E> shows multiple type parameters, where T represents success values and E represents error values. These generic enums are fundamental to Rust's type system, providing principled ways to handle nullable values and recoverable errors without exceptions or null pointers.

Each generic enum variant can use the type parameters differently or not at all. In Option<T>, Some uses T but None doesn't - None is the same for Option<i32> and Option<String>. In Result<T, E>, Ok uses T and Err uses E, allowing completely different types for success and failure cases. The enum as a whole is still monomorphized - Option<i32> and Option<String> are distinct types in the compiled binary. Pattern matching on generic enums is fully type-checked, ensuring you handle all variants and extract values with correct types.

Generic enums shine for modeling sum types - values that can be "this OR that" rather than "this AND that" (which structs model). Option models presence or absence, Result models success or failure, Either models one type or another. This explicit modeling of alternatives makes invalid states unrepresentable. You can't have a Result that's both Ok and Err, or an Option that's simultaneously Some and None. The type system enforces correct handling of all cases through exhaustive pattern matching.

In practice, generic enums are workhorses of idiomatic Rust code. Prefer Option<T> over null, Result<T, E> over exceptions, and custom generic enums for domain-specific alternatives. When designing generic enums, ensure variants represent truly distinct alternatives that can't coexist. Use pattern matching to handle variants - the compiler ensures exhaustiveness and correctness. Generic enums combined with pattern matching provide type-safe, zero-cost error handling and data modeling that would require complex type hierarchies in object-oriented languages."#,
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
            commentary: r#"Trait bounds constrain generic type parameters to types implementing specific traits, defining what operations are available on those types. T: Debug means "T can be any type that implements Debug," giving you access to debug formatting via {:?}. Without trait bounds, generic type parameters have no capabilities - you can only move, drop, or get their size, but not call methods, compare, clone, or perform any operations. Trait bounds add capabilities by requiring types to implement specific interfaces, making those interfaces' methods available.

This system inverts traditional object-oriented polymorphism. In OOP, types start with capabilities (inherited methods) and optionally specialize them. In Rust, types start blank and traits explicitly grant capabilities. This makes requirements explicit in function signatures rather than hidden in implementation details. When you see fn process<T: Debug + Clone>(value: T), you immediately know what operations the function performs on T. The signature is self-documenting - trait bounds are a contract between the function and its callers.

Trait bounds enable compile-time polymorphism with zero runtime cost. The compiler generates specialized versions of generic functions for each concrete type used, inlining trait methods and optimizing aggressively. There's no vtable lookup, no dynamic dispatch, no runtime type checking. The monomorphized code is as fast as handwritten type-specific code. This is fundamentally different from trait objects (dyn Trait) which use dynamic dispatch and have runtime overhead. Trait bounds on generics are a zero-cost abstraction.

In practice, use trait bounds to express exactly what capabilities your generic code needs. Start with minimal bounds and let the compiler guide you - it will error when you try to call unavailable methods. Common bounds include Debug for formatting, Clone for copying, PartialEq for comparison, and Send/Sync for thread safety. Combine bounds with + when you need multiple capabilities. Choose bounds carefully - fewer bounds make your code more reusable, more bounds make it more capable."#,
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
            commentary: r#"Where clauses provide a clearer syntax for complex trait bounds, moving them after the function signature rather than inline with type parameters. Instead of fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) which becomes hard to read, you write a clean signature followed by where T: Display + Clone, U: Clone + Debug. This significantly improves readability when functions have multiple type parameters or complex bound combinations. The where clause separates "what types" from "what constraints," making both easier to understand.

Where clauses are more than syntactic sugar - they enable expressing constraints that inline syntax can't. You can write bounds on concrete types (where i32: Display), on type constructors (where Vec<T>: Clone), and complex relationships between types (where T: AsRef<U>). Where clauses support lifetime bounds (where T: 'a meaning T must live at least as long as 'a) and constraints on associated types (where T: Iterator<Item = i32>). These advanced constraints are impossible or awkward with inline syntax alone.

The compiler treats inline bounds and where clauses identically - there's no performance difference, only readability. You can even mix them, though convention is to use one style consistently. Many style guides recommend where clauses for functions with multiple type parameters or multiple bounds per parameter, and inline bounds for simple single-bound cases. The goal is making function signatures scannable and understandable at a glance.

In practice, use where clauses to make complex generic signatures manageable. They're especially valuable in trait definitions where each associated function might have different complex bounds. Where clauses keep the signature focused on the function's logical shape while documenting requirements separately. This separation makes it easier to modify bounds without restructuring the entire signature. Modern IDEs provide excellent support for where clauses, showing expanded bounds on hover."#,
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
            commentary: r#"Methods on generic types can introduce additional type parameters beyond those defined on the struct, enabling powerful composition patterns. The mixup method on Point<T> introduces a new type parameter U, allowing it to accept a Point<U> with a potentially different type than T. The method signature uses both T (from self: Point<T>) and U (from other: Point<U>) to construct a new type Point<(T, U)> that combines them as tuples. This demonstrates how methods can be more generic than their containing types.

The type parameter scope rules are crucial here. T is defined on the struct Point<T> and available throughout all impl blocks for that type. U is defined only on the mixup method and exists only within that method's signature and body. Each method call can use different types for U - mixup could be called with Point<String>, Point<Vec<i32>>, or any other type. The struct's type parameter and method's type parameters are independent, allowing maximum flexibility in how methods combine and transform types.

This pattern enables powerful zero-cost abstractions. The compiler monomorphizes not just Point<T> but also each specific usage of mixup with its particular U types. If you call mixup with five different types, the compiler generates five optimized versions. Each is as efficient as if you'd hand-written a specific implementation for that type combination. There's no runtime type checking, no boxing, no indirection - just specialized, optimized machine code for each concrete combination.

In practice, method-level generic parameters are essential for operations that combine or transform types. They appear throughout the standard library: Iterator::map introduces the mapping function's type, collect() introduces the target collection type, and many parsing/conversion methods introduce the target type. This pattern lets methods be polymorphic in ways beyond their struct's type parameters, enabling fluent, composable APIs that maintain type safety and zero runtime cost."#,
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
            commentary: r#"Const generics parameterize types over compile-time constant values rather than types, enabling generic code that works with arrays of any size. The const N: usize syntax declares N as a compile-time constant parameter, making ArrayPair<T, N> generic over both the element type T and the array size N. This was impossible before Rust 1.51 - you'd need separate implementations for each array size or use runtime-sized collections like Vec. Const generics make arrays first-class citizens in generic programming, enabling zero-cost abstractions over fixed-size data.

Const generics are evaluated at compile time and become part of the type's identity. ArrayPair<i32, 3> and ArrayPair<i32, 5> are completely distinct types, just as different as ArrayPair<i32, 3> and ArrayPair<f64, 3>. The compiler generates separate monomorphized versions for each unique combination of type and constant parameters. The constant values are embedded directly into the generated code, enabling optimizations impossible with runtime sizing. The compiler knows exact array sizes, can unroll loops, and can optimize memory layout perfectly.

The syntax const N: usize looks like a variable but N is a compile-time constant, not a runtime variable. You can use N in type expressions ([T; N]), array initialization ([T::default(); N]), and as loop bounds. However, N must be known at compile time - you can't pass runtime values. The current implementation supports integer and boolean const parameters, with ongoing work to expand to more complex types. Const generics integrate with trait bounds, where clauses, and all other generic features.

In practice, const generics are invaluable for numerical and embedded programming where fixed-size arrays are common. They enable writing matrix libraries generic over dimensions, SIMD code generic over lane count, and network protocol handlers generic over packet sizes. All without runtime overhead, dynamic allocation, or size checking. The type system guarantees size compatibility at compile time - trying to combine incompatible array sizes results in clear compile errors rather than runtime crashes."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
