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
            commentary: r#"Traits define shared behavior through method signatures that types can implement, serving as Rust's interface mechanism and abstraction tool. A trait declares method signatures without implementations (though it can provide default implementations), and types opt-in to implementing traits. Any type implementing a trait must provide concrete implementations for all required methods. This is Rust's primary mechanism for polymorphism - multiple types can implement the same trait, allowing generic code to work with any type that implements the required traits.

Traits are more powerful than interfaces in many languages because they support retroactive implementation - you can implement traits on types you didn't define, including types from other crates or the standard library. This means you can extend existing types with new behavior without modifying their source code. Traits also support associated types, associated constants, and default method implementations that can call other trait methods. The trait system integrates deeply with Rust's type system, generics, and ownership model.

Unlike inheritance in object-oriented languages, traits are about capability rather than taxonomy. A type doesn't "is-a" something; it "can-do" something. You compose behavior by implementing multiple traits rather than inheriting from base classes. This composition over inheritance approach is more flexible - types can mix and match capabilities without being locked into rigid hierarchies. Traits also enable zero-cost abstractions through static dispatch with generic type parameters, unlike virtual methods which incur runtime overhead.

In practice, traits are everywhere in Rust. The standard library defines fundamental traits like Debug, Clone, Iterator, From, and many others. When designing APIs, define traits for shared behavior and implement them on your types. Use trait bounds on generic parameters to specify required capabilities. Traits make your code reusable, testable (through mock implementations), and composable. The trait system is what enables Rust's powerful generic programming while maintaining type safety and performance."#,
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
            commentary: r#"Traits can provide default implementations for methods, reducing code duplication when multiple types share common behavior. Types implementing the trait can use the default implementation or override it with their own specialized version. This example shows a trait with both a default method (speak) and a required method (introduce). Dog implements only the required method and automatically inherits the default speak implementation. This pattern lets you define common behavior once in the trait while requiring types to customize specific methods.

Default methods can call other trait methods, even those without default implementations. This enables powerful abstractions where the trait defines a complete API with some methods implemented in terms of others. The trait implementor only needs to provide the minimal set of required methods, and the default implementations build higher-level functionality on top. This is similar to template methods in object-oriented design, but more flexible since there's no inheritance hierarchy to navigate.

The ability to add default methods to traits enables trait evolution without breaking existing implementations. When you add a new method with a default implementation to an existing trait, all existing implementations continue to work without modification. They automatically get the new method with its default behavior. This is crucial for maintaining backward compatibility in library APIs. In contrast, adding a required method would be a breaking change, forcing all implementors to update their code.

In practice, use default implementations for methods that have sensible general behavior that most implementations can use. Override defaults only when a type needs specialized behavior. Common patterns include providing convenience methods implemented in terms of core methods (like Iterator::nth implemented using next), or defining algorithms that work for any implementor (like Iterator::collect). Default methods make traits more powerful and easier to implement while maintaining flexibility for customization."#,
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
            commentary: r#"Trait bounds constrain generic type parameters to types implementing specific traits, defining what operations are available in generic code. The syntax T: Printable means "T can be any type that implements Printable," giving the function access to all methods defined in the Printable trait. Without trait bounds, generic parameters have minimal capabilities - you can only move them, drop them, or get their size. Trait bounds add capabilities by requiring types to implement specific interfaces, making those interfaces' methods callable within the generic function.

This bounded polymorphism enables static dispatch and monomorphization. The compiler generates specialized versions of print_item for each concrete type used, with trait method calls resolved at compile time. For print_item(42), the compiler generates a version that directly calls i32's implementation of format(). This is fundamentally different from dynamic dispatch with trait objects (dyn Trait) where method calls go through vtables at runtime. Static dispatch via trait bounds achieves zero-cost abstraction - generic code is as fast as hand-written type-specific code.

Trait bounds make requirements explicit in function signatures, serving as both documentation and contract. When you see fn process<T: Clone + Send>(value: T), you immediately know the function clones values and works across threads. The type signature tells you what capabilities are required without reading the implementation. This explicitness aids understanding, enables better error messages when bounds aren't met, and helps the compiler optimize aggressively since it knows exactly what operations exist.

In practice, use trait bounds to express exactly what capabilities your generic code needs. Start minimal and let the compiler guide you - it will error when you try operations not covered by your bounds. Common trait bounds include Debug for printing, Clone for copying, PartialEq for comparison, Send for thread safety, and custom domain-specific traits. Multiple bounds combine with + syntax. Choose bounds thoughtfully - fewer bounds mean more reusable generic code, but you need enough bounds to do useful work."#,
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
            commentary: r#"Multiple trait bounds combine using the + syntax, requiring type parameters to implement all listed traits simultaneously. The bound T: Display + Debug + PartialOrd means T must implement all three traits, giving the function access to formatting (Display), debug output (Debug), and comparison operations (PartialOrd). This lets you combine orthogonal capabilities - a type can be both displayable and comparable without any relationship between those traits. The type parameter must satisfy every bound, and the function body can freely use methods from all of them.

Each trait bound adds capabilities independently. Display provides formatted output via {}, Debug provides debug output via {:?}, and PartialOrd provides comparison operators like >. These traits represent different aspects of a type's behavior, and combining them lets your generic function perform operations from all three domains. The compiler verifies at compile time that the concrete type used satisfies all bounds, generating specialized code that can inline and optimize method calls from all traits.

Multiple bounds can make signatures long and hard to read. When you have many type parameters or complex bound combinations, the where clause syntax (covered separately) provides clearer formatting. However, for simple cases with few bounds, inline + syntax is concise and readable. The compiler treats both syntaxes identically - there's no performance or semantic difference, only readability considerations. Choose the syntax that makes your specific signature clearest.

In practice, combine trait bounds to express complex requirements precisely. Common combinations include Clone + Send for thread-safe clonable data, Debug + Display for types that need both programmer and user-friendly output, and PartialEq + Eq + Hash for types used as hash map keys. Each additional bound restricts which types can be used but enables more operations in your generic code. Balance reusability (fewer bounds) against functionality (more bounds) based on what your code actually needs to do."#,
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
            commentary: r#"Trait objects enable dynamic dispatch by using dyn Trait as a type, allowing heterogeneous collections of different types that implement the same trait. The type &dyn Animal or Box<dyn Animal> represents "any type that implements Animal," erasing the concrete type at runtime. This lets you store Dog and Cat in the same Vec even though they're different types. The compiler can't know which type is in each box at compile time, so method calls use dynamic dispatch - looking up the correct implementation through a vtable at runtime.

Trait objects work through fat pointers containing two words: a pointer to the data and a pointer to a vtable (virtual method table). The vtable stores function pointers to the trait methods for the concrete type. When you call animal.make_sound(), the runtime follows the vtable pointer, looks up make_sound, and calls the appropriate implementation. This indirection has a small performance cost - one extra pointer dereference - but enables runtime polymorphism. Unlike generics which are monomorphized, trait objects allow code that works with multiple types without generating specialized versions for each.

Not all traits can be trait objects - they must be "object safe." A trait is object safe if all its methods have no type parameters, don't return Self, and have a receiver (like &self). Methods with type parameters can't be called through vtables because the compiler can't generate code for every possible type parameter at runtime. This restriction ensures trait objects can actually be implemented with vtables. Common traits like Clone aren't object safe because clone() returns Self, while Iterator is object safe.

In practice, use trait objects when you need heterogeneous collections or runtime polymorphism. They're essential for plugin systems, UI widgets, or any scenario where you don't know all implementing types at compile time. The performance cost is usually negligible compared to the flexibility gained. For performance-critical code with known types at compile time, use generic parameters with trait bounds instead. Balance the need for runtime flexibility against the benefits of compile-time optimization and type knowledge."#,
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
            commentary: r#"The impl Trait syntax allows returning types that implement a trait without naming the concrete type, hiding implementation details while maintaining static dispatch. When you write fn create_shape() -> impl Shape, you're saying "this returns something that implements Shape" without exposing whether it's Circle, Rectangle, or some complex type. The concrete type is determined at compile time and fixed for each function. This is purely a signature feature - internally the function returns a specific type, but callers only see the trait interface.

Unlike trait objects (Box<dyn Trait>), impl Trait uses static dispatch with zero runtime cost. The compiler knows the actual return type and generates specialized code at each call site. There's no boxing, no vtable, no indirection - just direct function calls with full optimization. The returned value is stored directly where needed, not heap-allocated. This makes impl Trait perfect for returning iterators, closures, and other complex types where exposing the concrete type would be verbose or encapsulation-breaking.

The key limitation is that you must return a single concrete type from all code paths. You can't return Circle from one branch and Rectangle from another - the return type must be the same concrete type throughout the function, even though callers don't see what it is. If you need to return different types from different branches, you must use trait objects (Box<dyn Trait>) or enums. This restriction exists because the compiler must know the exact return type size and layout at compile time.

In practice, impl Trait is ideal for returning iterators (impl Iterator<Item = i32>), closures, or other complex generated types whose exact names are unwieldy. It's common in APIs where implementation details shouldn't leak into the public interface. The caller gets a clean signature showing what capabilities the returned value has, while you maintain freedom to change the concrete implementation without breaking the API. This encapsulation combined with zero-cost abstraction makes impl Trait valuable for library design."#,
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
            commentary: r#"The derive attribute automatically generates trait implementations for common traits, saving boilerplate and reducing errors. When you write #[derive(Debug, Clone, PartialEq)], the compiler generates implementations based on the struct's definition. For Debug, it creates output showing the struct name and all fields. For Clone, it clones each field. For PartialEq, it compares all fields for equality. These generated implementations follow sensible defaults that work for most cases, letting you opt into common functionality without writing repetitive code.

Derive works through procedural macros that inspect your type's structure at compile time and generate appropriate code. For Clone, it generates code that clones each field, so derive only works if all fields implement Clone. For Copy (which requires Clone), it marks the type as bitwise copyable, requiring all fields to be Copy. The derive system ensures correctness - you can't accidentally derive traits that don't make sense for your type's structure. The generated code is inlined and optimized like hand-written implementations.

The derivable traits in the standard library cover common needs: Debug for formatting, Clone and Copy for duplication, PartialEq and Eq for equality, PartialOrd and Ord for ordering, Hash for hash maps, and Default for default values. Each has specific requirements - Eq requires PartialEq, Ord requires Eq and PartialOrd, Copy requires Clone. The compiler enforces these relationships. Custom derive macros can add domain-specific derivable traits, popular in libraries for serialization (Serialize, Deserialize) and other boilerplate-heavy patterns.

In practice, derive common traits unless you need custom logic. Use Debug for all types during development, Clone when you need explicit copying, PartialEq for comparability, and Hash + Eq for hash map keys. Derive Copy only for simple stack-allocated types where bitwise copy is appropriate. When derived implementations don't match your needs, implement traits manually. The ability to mix derived and manual implementations lets you use defaults where they work and customize where they don't."#,
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
            commentary: r#"Associated types define placeholder types within traits that implementers must specify, creating a tight relationship between the trait and a related type. The syntax type Item; in the trait declares an associated type that implementations must specify with type Item = SomeConcreteType;. Methods in the trait can then use Self::Item in their signatures. This is different from generic type parameters - with associated types, each type can implement the trait only once with a specific associated type, while with generics, a type could implement Trait<TypeA> and Trait<TypeB> as distinct trait implementations.

The single-implementation constraint makes associated types appropriate when there's one natural choice for the associated type. For Iterator, each iterator type has one obvious item type - Vec<i32>::IntoIter produces i32 items, String::Chars produces char items. Making Item an associated type rather than a generic parameter means users write Iterator instead of Iterator<Item = i32>, reducing verbosity. The associated type is determined by the implementing type, not by the caller. This makes APIs cleaner when the associated type is an output determined by the trait implementation.

Associated types interact with trait bounds in powerful ways. You can write where T: Iterator<Item = i32> to constrain not just that T is an Iterator, but that it produces i32 specifically. This level of precision is crucial for generic code that needs to constrain related types. Associated types are also useful for traits with complex type relationships where multiple related types need to be coordinated. The trait can define multiple associated types that work together while keeping the trait name simple.

In practice, use associated types for types that are outputs or results of the trait's functionality - types determined by the implementation rather than chosen by the caller. Iterator::Item, Future::Output, and AsRef::Target are canonical examples. If callers need to parameterize over the type (like choosing what to store in a container), use generic parameters instead. The distinction: generic parameters are inputs to the trait, associated types are outputs determined by the implementation. Choose based on whether multiple implementations make sense for the same type."#,
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
            commentary: r#"Operator overloading in Rust works through traits from std::ops module, making operators like +, -, *, and / available for custom types. Implementing Add for your type lets you use the + operator, Sub for -, Mul for *, and so on. The Add trait defines fn add(self, rhs: Self) -> Self::Output where Output is an associated type specifying the result type. This system is more restricted than languages like C++ - you can only overload predefined operators by implementing their corresponding traits, not create entirely new operators. This restriction prevents operator abuse while enabling natural syntax for mathematical and domain-specific types.

The Output associated type provides flexibility in what operations produce. For Point + Point, Output might be Point, but for Matrix * Vector, Output might be Vector rather than Matrix. The type system enforces correctness - if your add implementation's Output type is Point, using + must produce a Point. This type-level checking catches errors at compile time that would be runtime bugs in dynamically typed languages. The operators also respect Rust's ownership semantics - add takes ownership by default, though you can implement Add for references to avoid moves.

Rust's operator overloading is explicit and opt-in. Unlike languages where operators might hide expensive operations, in Rust you must explicitly implement traits to enable operators. When you see x + y, you know either it's a primitive operation or someone intentionally implemented Add. This explicitness makes code behavior more predictable. The compiler also generates optimized code for operator calls, inlining them like any other method, so there's no performance penalty for using operators versus method calls.

In practice, implement operator traits for types where operators have natural mathematical or domain meaning. Vectors, matrices, complex numbers, and units of measurement are good candidates. Avoid overloading operators for surprising behaviors - + should feel like addition, not something arbitrary. Consider implementing operators for both owned values and references to avoid forcing unnecessary moves. Most operator traits come with assignment variants (AddAssign for +=) that you can implement for in-place operations. Consistent operator implementations make code intuitive and maintainable."#,
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
            commentary: r#"Supertraits express that one trait depends on another, requiring implementers to implement both traits. The syntax trait PrintableShape: Display means "any type implementing PrintableShape must also implement Display first." This lets PrintableShape methods use Display methods (like self in a format string) knowing they're available. Supertraits build on existing traits, creating a hierarchy of capabilities where more specific traits extend more general ones. This is similar to interface inheritance in other languages but focuses on requirements rather than implementation inheritance.

The supertrait relationship is enforced at compile time. You cannot implement PrintableShape without also implementing Display. The compiler checks all supertrait bounds are satisfied before allowing trait implementation. This ensures methods in PrintableShape can safely call Display methods without runtime checks. The bounds propagate transitively - if PrintableShape: Display and Display: Debug, then implementing PrintableShape implicitly requires Debug as well. This creates a lattice of traits with clear dependency relationships.

Supertraits enable building sophisticated trait hierarchies that model domain concepts. Standard library examples include Eq: PartialEq (full equality requires partial equality), Ord: PartialOrd (total ordering requires partial ordering), and Copy: Clone (bitwise copying requires cloning). Each more specific trait adds additional guarantees or capabilities beyond its supertrait. The system maintains clear semantics - implementing a supertrait doesn't automatically implement the subtrait, you must explicitly opt into each trait's additional guarantees.

In practice, use supertraits when your trait genuinely depends on another trait's functionality. Don't create deep hierarchies unnecessarily - keep trait dependencies minimal and meaningful. Supertraits make APIs self-documenting by explicitly stating dependencies in trait definitions rather than hiding them in method bounds. Common patterns include requiring Debug on custom traits for logging, Display for user-facing types, or Send + Sync for concurrent types. Balance the convenience of supertrait methods against the restriction that implementers must implement all traits in the chain."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
