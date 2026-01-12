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
            commentary: r#"📚 INTRODUCTION
Traits define shared behavior through method signatures
that types can implement. A trait declares what methods a
type must provide without specifying how, and types opt-in
by implementing those methods. This example shows a Greet
trait with one method, and Person implementing that trait
to provide concrete greeting behavior.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
In languages like Java or C#, interfaces solve similar
problems but lack retroactive implementation - you can't
implement interfaces on types you don't own. Go uses
implicit interfaces but without compile-time verification
of complete implementation. Traits solve: (1) Safe
polymorphism without runtime overhead (2) Extending
existing types from other crates (3) Explicit
capability-based design over inheritance (4) Compile-time
verification that all required methods are implemented.

🔍 IMPORTANT DETAILS & INTRICACIES
Traits use static dispatch by default, meaning method calls
are resolved at compile time with zero runtime cost. Unlike
virtual methods in C++ or interface calls in Java, trait
methods are monomorphized - the compiler generates
specialized code for each concrete type. Traits can contain
method signatures, default implementations, associated
types, and associated constants. The orphan rule prevents
conflicting implementations by requiring that either the
trait or the type be defined in your crate.

💼 WHERE IT'S MOST USED
Standard library traits power Rust's core functionality:
Debug for printing, Clone for copying, Iterator for
iteration, From/Into for conversions, Send/Sync for thread
safety. Custom traits define domain interfaces -
Serialize/Deserialize in serde, Query in database
libraries, Handler in web frameworks. Traits enable
dependency injection through trait bounds, making code
testable with mock implementations.

✅ TAKEAWAY
Traits are Rust's fundamental abstraction mechanism,
defining "what a type can do" rather than "what a type is."
They enable safe polymorphism, code reuse, and powerful
generic programming while maintaining zero-cost
abstractions through compile-time dispatch. Master traits
to write flexible, composable Rust code."#,
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
            commentary: r#"📚 INTRODUCTION
Traits can provide default method implementations that
types inherit automatically, reducing boilerplate when
multiple types share common behavior. Types can either use
the default or override it with specialized logic. This
example shows a trait mixing default (speak) and required
(introduce) methods, with Dog inheriting the default speak
behavior.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Java interfaces before Java 8 couldn't have default
methods, requiring all implementations to rewrite identical
code. C++ abstract classes mix implementation with
inheritance hierarchies. Default trait methods solve: (1)
Code reuse without inheritance complexity (2) Trait
evolution without breaking existing implementations (3)
Separation of required minimal interface from convenient
helper methods (4) Backward compatibility when extending
trait APIs.

🔍 IMPORTANT DETAILS & INTRICACIES
Default methods can call other trait methods, even required
ones without defaults, enabling the template method
pattern. When you add a default method to an existing
trait, all implementations automatically gain that method
without code changes - a non-breaking addition. The
compiler includes default method code only in types that
don't override it. Default methods have full access to self
and can be as complex as needed, including generic
parameters and where clauses.

💼 WHERE IT'S MOST USED
Iterator trait extensively uses defaults - only next() is
required, but nth(), count(), collect(), map(), filter()
and 70+ methods are provided as defaults. Error trait
provides default implementations for source() and
description(). Display implementations often use default
implementations for more complex formatting. Custom domain
traits use defaults for convenience methods built on core
operations.

✅ TAKEAWAY
Default trait methods balance convenience with flexibility
- implementors provide only essential methods while gaining
a rich API automatically. This enables trait evolution,
reduces boilerplate, and supports the minimal-yet-complete
interface design pattern. Use defaults for genuinely common
behavior, leaving core operations as required methods."#,
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
            commentary: r#"📚 INTRODUCTION
Trait bounds constrain generic type parameters to types
implementing specific traits, defining what operations are
available. The syntax T: Printable means "T can be any type
implementing Printable," giving the function access to all
Printable methods. This example shows a generic function
that works with any printable type.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
C++ templates allow any operations, catching errors late
when instantiated. Go's interface{} provides no
compile-time guarantees. Java's bounded generics are close
but tied to inheritance. Trait bounds solve: (1)
Compile-time verification that generic code can perform
required operations (2) Zero-cost abstraction through
monomorphization (3) Explicit capability requirements
visible in signatures (4) Better error messages pinpointing
where bounds aren't satisfied.

🔍 IMPORTANT DETAILS & INTRICACIES
Trait bounds enable monomorphization - the compiler
generates a specialized version of the function for each
concrete type used. For print_item(42), the compiler
creates a version calling i32's format() directly, with
zero runtime overhead. Without bounds, generic parameters
can only be moved, dropped, or sized. The compiler
aggressively optimizes bounded generic code, often inlining
trait method calls completely. Bounds create compile-time
contracts that enable both safety and performance.

💼 WHERE IT'S MOST USED
Generic collection methods use trait bounds extensively -
Vec::sort requires Ord, HashMap::insert requires Hash + Eq.
Serialization libraries bound generic types with Serialize.
Async functions use Future bounds. Numeric libraries bound
types with arithmetic traits. Any generic algorithm that
operates on values rather than just storing them uses trait
bounds to specify required capabilities.

✅ TAKEAWAY
Trait bounds make generic code both safe and fast by
declaring required capabilities explicitly in function
signatures. The compiler verifies bounds at compile time
and generates optimized specialized code for each type. Use
minimal bounds for maximum reusability, adding only what
operations your code actually needs to perform."#,
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
            commentary: r#"📚 INTRODUCTION
Multiple trait bounds combine using + syntax, requiring
type parameters to implement all listed traits
simultaneously. The bound T: Display + Debug + PartialOrd
means T must provide formatting, debug output, and
comparisons. This example shows a function requiring three
independent capabilities from its generic parameter.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Single trait bounds are often insufficient for real-world
generic code. Java allows multiple interface bounds but
ties them to inheritance. C# uses where constraints
similarly. Multiple bounds solve: (1) Expressing complex
capability requirements precisely (2) Combining orthogonal
traits without coupling them (3) Compile-time verification
that all needed operations exist (4) Enabling generic code
to perform diverse operations safely.

🔍 IMPORTANT DETAILS & INTRICACIES
Each trait bound adds independent capabilities - Display
gives {}, Debug gives {:?}, PartialOrd gives comparison
operators. The compiler verifies all bounds at compile time
and generates monomorphized code accessing all traits'
methods efficiently. Long signatures with many bounds can
use where clause syntax for readability: fn compare<T>(a:
T, b: T) where T: Display + Debug + PartialOrd. Both
syntaxes are semantically identical with zero performance
difference.

💼 WHERE IT'S MOST USED
HashMap keys require Hash + Eq. Thread-safe data structures
bound generics with Send + Sync. Serialization often needs
Clone + Debug + Serialize. Generic numeric code combines
arithmetic traits like Add + Sub + Mul. Error handling
generics often require Display + Debug + Error. Any complex
generic algorithm likely needs multiple orthogonal
capabilities combined.

✅ TAKEAWAY
Multiple trait bounds let you precisely specify all
capabilities generic code needs, combining independent
traits without coupling them. Balance reusability (fewer
bounds) against functionality (more bounds) by requiring
only what your code actually uses. The + syntax composes
capabilities cleanly while maintaining compile-time
verification and zero-cost abstractions."#,
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
            commentary: r#"📚 INTRODUCTION
Trait objects enable dynamic dispatch using dyn Trait,
allowing heterogeneous collections of different types
implementing the same trait. Box<dyn Animal> means "any
type implementing Animal," erasing the concrete type at
runtime. This example stores Dog and Cat together in a Vec,
calling their methods through runtime dispatch.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Static dispatch via generics requires knowing all types at
compile time. Java interfaces use dynamic dispatch
everywhere, incurring overhead even when unnecessary. C++
virtual functions provide runtime polymorphism but through
inheritance. Trait objects solve: (1) Heterogeneous
collections of different types (2) Runtime polymorphism
without inheritance hierarchies (3) Plugin systems where
types aren't known at compile time (4) Balancing
flexibility with controlled performance cost.

🔍 IMPORTANT DETAILS & INTRICACIES
Trait objects are fat pointers - two words containing a
data pointer and vtable pointer. The vtable stores function
pointers to trait method implementations for the concrete
type. Method calls dereference the vtable, adding one
indirection compared to static dispatch. Not all traits are
object-safe: methods can't have type parameters, return
Self, or lack receivers. Clone isn't object-safe (returns
Self), but Debug and Display are. The ?Sized bound is
implicit for trait objects.

💼 WHERE IT'S MOST USED
Error handling uses Box<dyn Error> for heterogeneous error
types. GUI frameworks store widgets as Box<dyn Widget>.
Plugin systems use trait objects for runtime-loaded code.
Middleware pipelines store handlers as Vec<Box<dyn
Handler>>. Game engines use trait objects for entity
components. Any system needing runtime polymorphism or
heterogeneous collections of related types uses trait
objects.

✅ TAKEAWAY
Trait objects trade compile-time type knowledge for runtime
flexibility, enabling heterogeneous collections and dynamic
dispatch through vtables. Use them when you need runtime
polymorphism or don't know all types at compile time.
Prefer static dispatch with generics for known types at
compile time. The performance cost is small but real - one
vtable lookup per call."#,
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
            commentary: r#"📚 INTRODUCTION
The impl Trait syntax returns types implementing a trait
without naming the concrete type, hiding implementation
details while maintaining static dispatch. The signature fn
create_shape() -> impl Shape says "returns something
implementing Shape" without exposing whether it's Circle,
Rectangle, or a complex generated type.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Without impl Trait, returning closures or complex iterators
requires writing unwieldy type names like impl Fn(i32) ->
i32 or fully spelling out iterator adapter types. Trait
objects (Box<dyn Trait>) work but add heap allocation and
vtable overhead. impl Trait solves: (1) Hiding complex
return types while preserving static dispatch (2) Zero-cost
abstraction for iterators and closures (3) API
encapsulation without exposing implementation details (4)
Avoiding heap allocation and runtime overhead of trait
objects.

🔍 IMPORTANT DETAILS & INTRICACIES
impl Trait uses static dispatch - the compiler knows the
actual concrete type and optimizes accordingly, generating
the same code as if you wrote the type explicitly. No
boxing, vtables, or heap allocation occurs. The returned
value lives directly in the caller's stack frame. Major
limitation: all code paths must return the same concrete
type. You can't return Circle from one branch and Rectangle
from another. The compiler must know the exact size and
layout at compile time.

💼 WHERE IT'S MOST USED
Returning iterators without spelling complex adapter
chains: fn numbers() -> impl Iterator<Item=i32>. Returning
closures from functions without Box. Async functions
implicitly return impl Future. API design where exposing
concrete types would break encapsulation or create
maintenance burden. Any situation where you want
caller-side abstraction with producer-side static dispatch.

✅ TAKEAWAY
impl Trait provides zero-cost abstraction for return types,
hiding implementation details while preserving all static
dispatch optimizations. Use it for iterators, closures, and
complex types you don't want to expose. Remember: all
return paths must return the same concrete type. For truly
dynamic polymorphism, use trait objects instead."#,
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
            commentary: r#"📚 INTRODUCTION
The derive attribute automatically generates trait
implementations for common traits, eliminating boilerplate
code. Writing #[derive(Debug, Clone, PartialEq)] instructs
the compiler to generate Debug, Clone, and PartialEq
implementations based on the struct's fields. This example
shows Point deriving three traits and using their methods
without manually implementing them.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Manually implementing Debug, Clone, PartialEq for every
struct creates repetitive boilerplate prone to errors and
maintenance burden. Java requires verbose toString(),
equals(), hashCode() methods. Python's dataclasses reduce
boilerplate but lack compile-time checks. Derive solves:
(1) Eliminating repetitive trait implementations (2)
Automatic correctness based on field types (3) Maintaining
consistency when adding/removing fields (4) Enabling opt-in
functionality with one attribute.

🔍 IMPORTANT DETAILS & INTRICACIES
Derive uses procedural macros that analyze type structure
at compile time. For Clone, it generates code cloning each
field, so derive only works if all fields implement Clone.
Copy requires all fields to be Copy and automatically
implements Clone. The compiler enforces trait dependencies:
Eq requires PartialEq, Ord requires PartialOrd + Eq.
Generated code is optimized identically to hand-written
implementations. Custom derive macros enable
domain-specific traits like Serialize/Deserialize.

💼 WHERE IT'S MOST USED
Nearly all structs derive Debug for development debugging.
Data structures derive Clone for explicit copying. Value
types derive PartialEq for comparisons. HashMap keys derive
Hash + Eq. Ordered collections require Ord. Serde
extensively uses derive(Serialize, Deserialize) for
automatic serialization. Configuration structs often derive
Default. Any struct representing data typically derives
multiple standard traits.

✅ TAKEAWAY
Derive traits to eliminate boilerplate for common
functionality, letting the compiler generate correct
implementations based on field types. Start by deriving
Debug on all types, then add Clone, PartialEq, and others
as needed. The derive system ensures correctness through
compile-time checks and dependencies. Implement traits
manually only when you need custom logic that differs from
field-wise operations."#,
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
            commentary: r#"📚 INTRODUCTION
Associated types define placeholder types within traits
that implementers specify, creating relationships between
traits and related types. The syntax type Item; declares a
placeholder that implementations set with type Item =
ConcreteType. This example shows a Container trait with an
associated Item type that methods reference.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Generic traits like Trait<T> allow multiple implementations
per type (MyType could implement Trait<i32> and
Trait<String>). This flexibility is sometimes unwanted
complexity. Java and C# use generic interfaces but can't
restrict to single implementations. Associated types solve:
(1) Ensuring one natural implementation per type (2)
Cleaner syntax - Iterator not Iterator<Item=T> (3) Output
types determined by implementation, not caller (4)
Constraining specific associated types in bounds: T:
Iterator<Item = i32>.

🔍 IMPORTANT DETAILS & INTRICACIES
Associated types enforce single implementation:
NumberContainer can only implement Container once, fixing
Item = i32. The implementing type determines the associated
type, not the caller. You can constrain associated types in
bounds: fn process<T: Iterator<Item = String>>(iter: T).
Traits can have multiple associated types that coordinate
with each other. Associated types appear in method
signatures as Self::Item. The choice between generic
parameters and associated types fundamentally affects API
design.

💼 WHERE IT'S MOST USED
Iterator uses Item for element type - each iterator has
exactly one item type. Future uses Output for async result
type. Deref uses Target for dereferenced type. Add uses
Output for addition result type. Index uses Output for
indexed element type. AsRef/AsMut use Target for reference
target. Any trait where the related type is naturally
determined by the implementing type uses associated types.

✅ TAKEAWAY
Associated types express output types determined by trait
implementations, not callers. Use them when each type
should implement the trait exactly once with a natural
associated type. Use generic parameters when callers need
to choose the type or multiple implementations make sense.
Associated types simplify APIs and enable precise type
constraints in generic bounds."#,
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
            commentary: r#"📚 INTRODUCTION
Operator overloading in Rust works through std::ops traits,
enabling custom types to use operators like +, -, *, /.
Implementing Add for a type enables the + operator. This
example implements Add for Point, allowing p1 + p2 syntax
with the Output associated type specifying the result.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
C++ allows overloading any operator, leading to abuse like
<< for stream output. Python allows operator overloading
but with potential performance surprises. Languages without
overloading force verbose method calls for mathematical
types. Operator traits solve: (1) Natural mathematical
syntax for custom types (2) Preventing operator abuse
through fixed trait definitions (3) Type-safe operations
verified at compile time (4) Respecting ownership semantics
in operator implementations.

🔍 IMPORTANT DETAILS & INTRICACIES
Add trait defines fn add(self, rhs: Self) -> Self::Output.
The Output associated type allows flexible result types -
Point + Point = Point, but Matrix * Vector could equal
Vector. Operators take ownership by default, but you can
implement Add for &Point to use references. The compiler
inlines operator calls identically to method calls, so no
performance penalty exists. Assignment variants (AddAssign
for +=) enable in-place operations. You can only overload
predefined operators, not create new ones.

💼 WHERE IT'S MOST USED
Mathematical types: vectors, matrices, complex numbers,
quaternions. Units of measurement libraries (meters +
meters). Arbitrary precision arithmetic types. Path and
string concatenation. Bitwise operations on custom bit
sets. Domain-specific numeric types like currency or
coordinates. Any type where operator syntax provides
clearer, more natural code than method calls for
mathematical or logical operations.

✅ TAKEAWAY
Operator overloading via traits brings natural mathematical
syntax to custom types while preventing abuse through
predefined operator traits. Implement operators where they
have clear, intuitive meaning aligned with mathematical or
domain conventions. The Output associated type enables
flexible result types while maintaining compile-time type
safety. Consider implementing both owned and reference
versions to avoid unnecessary moves."#,
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
            commentary: r#"📚 INTRODUCTION
Supertraits express that one trait depends on another using
the syntax trait SubTrait: SuperTrait. The definition trait
PrintableShape: Display means any type implementing
PrintableShape must first implement Display. This example
shows PrintableShape methods using Display's capabilities
within the describe method.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Without supertraits, every method needing Display would
need individual bounds like fn describe(&self) where Self:
Display, creating repetition. Java and C# support interface
inheritance but blur requirements with implementation.
Supertraits solve: (1) Declaring trait dependencies once in
the trait definition (2) Enabling trait methods to call
supertrait methods safely (3) Building capability
hierarchies with clear relationships (4) Making API
requirements explicit and self-documenting.

🔍 IMPORTANT DETAILS & INTRICACIES
Supertrait bounds are enforced at compile time - you cannot
implement PrintableShape without implementing Display
first. Dependencies propagate transitively: if A: B and B:
C, then implementing A requires C. However, implementing a
supertrait doesn't automatically implement the subtrait -
you must explicitly opt into each. The compiler checks all
bounds before allowing implementation. Supertraits create a
lattice of capabilities, not a tree - traits can have
multiple supertraits.

💼 WHERE IT'S MOST USED
Standard library examples: Eq: PartialEq (total equality
requires partial), Ord: PartialOrd + Eq (total ordering
requires partial ordering and equality), Copy: Clone
(bitwise copy requires clone). Custom error types often
require Display + Debug. Thread-safe traits require Send +
Sync. Serialization traits may require Clone + Debug.
Domain-specific trait hierarchies model progressively more
specific capabilities.

✅ TAKEAWAY
Supertraits declare dependencies between traits, ensuring
implementers provide all required capabilities and letting
trait methods safely call supertrait methods. Use them to
build capability hierarchies where specific traits extend
general ones with additional guarantees. Keep hierarchies
shallow and meaningful - each supertrait should genuinely
be required by the subtrait's functionality. Supertraits
make API requirements explicit and self-documenting."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
