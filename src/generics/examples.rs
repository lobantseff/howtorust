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
            commentary: r#"📚 INTRODUCTION
Generic functions use type parameters (like <T>) to work
with multiple types while maintaining type safety. The
largest function can find the largest element in a slice of
any comparable type - integers, characters, or custom
types.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike C++ templates (text substitution) or Java generics
(runtime type erasure), Rust generics are monomorphized at
compile time - the compiler generates specialized versions
for each concrete type used. This prevents: runtime type
errors, performance overhead from dynamic dispatch, and
code duplication from manually writing type-specific
functions. Compared to dynamic languages, you get type
safety without runtime checks.

🔍 IMPORTANT DETAILS & INTRICACIES
The trait bound T: PartialOrd is essential - without it,
the > operation is unavailable. Generic parameters start
with zero capabilities; trait bounds grant them.
Monomorphization means largest::<i32> and largest::<char>
are literally different functions in the binary, each
optimized for its type. This has zero runtime overhead but
can increase binary size if many types are used. The
compiler inlines and specializes aggressively because it
knows exact types.

💼 WHERE IT'S MOST USED
Standard library collections (Vec::sort, Iterator::max),
serialization libraries (serde generic over types),
database query builders, and any API that needs type-safe
reusability without performance cost. Common in systems
programming where both abstraction and zero-cost are
required.

✅ TAKEAWAY
Generic functions let you write one implementation that
works with any type satisfying specified trait bounds,
compiled into specialized, optimized code for each concrete
type used. You get the reusability of dynamic typing with
the performance and safety of static typing, making
generics fundamental to Rust's zero-cost abstraction
philosophy."#,
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
            commentary: r#"📚 INTRODUCTION
Generic structs use type parameters to create reusable data
structures that can hold values of any type. Point<T>
declares that both x and y coordinates must be the same
type T, enabling type-safe geometric operations.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike C++ templates or Java generics, Rust's generic
structs are fully type-checked at definition and
monomorphized at compile time. This prevents: mixing
incompatible types (Point { x: 5, y: 4.0 } is a compile
error), runtime type checking overhead, and the need to
write separate structs for each type. Compared to languages
with runtime generics, you get compile-time safety with
zero performance cost.

🔍 IMPORTANT DETAILS & INTRICACIES
The single type parameter T enforces that x and y have
identical types - the compiler can't unify i32 and f64.
Point<i32> and Point<f64> are completely distinct types in
the compiled binary, each with its own memory layout known
at compile time. No boxing, no type tags, no indirection -
just efficient machine code. Field access is as fast as
non-generic structs because types are resolved at compile
time.

💼 WHERE IT'S MOST USED
Core to standard library: Vec<T> for growable arrays,
HashMap<K, V> for key-value stores, Option<T> for nullable
values, Result<T, E> for error handling. Used in game
development for generic math types, in parsers for abstract
syntax trees, and anywhere type-safe containers are needed
without runtime overhead.

✅ TAKEAWAY
Generic structs enable writing one data structure
definition that works with any type, with the compiler
generating specialized, optimized versions for each
concrete type used. The struct definition acts as a
compile-time contract about type relationships, preventing
invalid combinations while maintaining zero runtime cost -
a cornerstone of Rust's type system."#,
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
            commentary: r#"📚 INTRODUCTION
Multiple type parameters allow structs to hold fields of
different, independent types. Pair<T, U> enables first and
second to have completely unrelated types, creating
flexible heterogeneous data structures like key-value pairs
or success/error combinations.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike single-parameter generics that enforce type
uniformity, multiple parameters enable type heterogeneity.
This prevents: using runtime type systems to mix different
types, losing type safety with void pointers or Object
types, and duplicating code for different type
combinations. Compared to dynamically typed tuples, you get
compile-time verification of which types go where.

🔍 IMPORTANT DETAILS & INTRICACIES
Each combination of concrete types creates a distinct
monomorphized version - Pair<i32, String> and Pair<String,
i32> are different types with different memory layouts. The
compiler generates optimized code for each unique
combination actually used. However, too many type
parameters increase complexity and cognitive load. Consider
if associated types would better express relationships,
like Iterator using Item = T rather than Iterator<T>.

💼 WHERE IT'S MOST USED
HashMap<K, V> for key-value stores (independent types for
keys and values), Result<T, E> for operations with success
type T and error type E, tuple types (T, U, V), and
Either<L, R> for sum types. Common in parsers combining
token and value types, state machines pairing state and
data, and APIs bridging different type domains.

✅ TAKEAWAY
Multiple type parameters enable structs to combine
independent types in a single data structure, with each
type combination compiled into specialized, optimized code.
Choose multiple parameters when fields are truly
independent (like HashMap keys and values), but prefer
simpler designs or associated types when relationships
exist - balancing expressiveness with usability."#,
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
            commentary: r#"📚 INTRODUCTION
Methods on generic types can be implemented generically
(for all type parameters) or specialized (for specific
concrete types). impl<T> Point<T> provides methods for any
type T, while impl Point<f32> provides methods only for f32
points, enabling both universal and type-specific
functionality.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike inheritance-based polymorphism where you override
inherited methods, Rust lets you add entirely new
capabilities to specific type instantiations. This
prevents: forcing all types to support operations only
meaningful for some, runtime method lookup overhead, and
the fragile base class problem. Compared to languages with
single inheritance, you can provide completely different
method sets for different type parameters.

🔍 IMPORTANT DETAILS & INTRICACIES
The impl<T> syntax declares T as a type parameter for the
impl block - without it, the compiler looks for a concrete
type named T. Each monomorphized version gets its own
compiled methods: Point<i32>::x() and Point<f64>::x() are
different functions in the binary. Specialized impl blocks
add methods only available for specific types -
distance_from_origin() exists only on Point<f32>, not
Point<i32>, enforced at compile time.

💼 WHERE IT'S MOST USED
Standard library: Vec<T> has generic methods like push()
for any T, but Vec<u8> has specialized from_utf8() for byte
strings. Generic collections providing universal access
methods plus type-specific operations. Numeric libraries
with specialized SIMD methods for primitive types. Anywhere
you need both broad reusability and type-specific
optimizations.

✅ TAKEAWAY
Generic method implementations provide universal
functionality across all type parameters, while specialized
implementations add capabilities unique to specific types,
all resolved at compile time with zero overhead. This
enables designing APIs that are maximally reusable while
still leveraging type-specific features - you get the best
of both abstraction and specialization."#,
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
            commentary: r#"📚 INTRODUCTION
Generic enums use type parameters to create tagged unions
that work with any type. Option<T> models presence
(Some(T)) or absence (None), while Result<T, E> models
success (Ok(T)) or failure (Err(E)), replacing null
pointers and exceptions with type-safe alternatives.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike null pointers (causing crashes) or exceptions
(invisible in type signatures), generic enums make
alternatives explicit and compiler-checked. This prevents:
null pointer dereferencing, uncaught exceptions, forgetting
to handle error cases, and representing impossible states.
Compared to Java's Optional or checked exceptions, Rust's
approach is zero-cost and enforced through exhaustive
pattern matching.

🔍 IMPORTANT DETAILS & INTRICACIES
Variants can use type parameters differently - in
Option<T>, Some uses T but None doesn't (None is the same
across all Option types). Each concrete parameterization is
monomorphized into a distinct type: Option<i32> and
Option<String> are different types in the binary. Pattern
matching is exhaustive - the compiler errors if you don't
handle all variants. Memory layout is optimized -
Option<&T> has the same size as &T due to niche
optimization.

💼 WHERE IT'S MOST USED
Option<T> replaces nullable references throughout Rust
code. Result<T, E> is the standard error-handling mechanism
(file I/O, parsing, network operations). Either<L, R> for
values that can be one type or another. Custom generic
enums for domain-specific state machines, AST nodes in
parsers, and anywhere you need type-safe sum types.

✅ TAKEAWAY
Generic enums model "this OR that" relationships with
compile-time exhaustiveness checking and zero runtime
overhead, making invalid states unrepresentable. Combined
with pattern matching, they provide type-safe error
handling and data modeling superior to null pointers or
exceptions, forming a cornerstone of idiomatic Rust code
that's both safer and faster than traditional approaches."#,
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
            commentary: r#"📚 INTRODUCTION
Trait bounds constrain generic type parameters to types
implementing specific traits, defining what operations are
valid. T: Debug means "T must implement Debug," granting
access to {:?} formatting. Without bounds, generic
parameters have zero capabilities beyond move/drop.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike object-oriented inheritance where types start with
capabilities and specialize, Rust types start blank and
traits grant capabilities. This prevents: calling methods
that don't exist, hidden requirements in implementation,
and runtime method lookup overhead. Compared to duck
typing, you get compile-time verification. Compared to
interfaces in Java/C#, you get zero runtime cost through
monomorphization instead of vtables.

🔍 IMPORTANT DETAILS & INTRICACIES
Trait bounds are compile-time polymorphism - the compiler
generates specialized versions for each concrete type,
inlining trait methods aggressively. No vtable lookup, no
runtime dispatch, no type checking at runtime. This differs
from trait objects (dyn Trait) which have runtime overhead.
The signature fn process<T: Debug + Clone>(value: T) is
self-documenting - you know exactly what operations the
function needs. Bounds are a contract enforced by the
compiler.

💼 WHERE IT'S MOST USED
Collections requiring comparison (T: Ord for sorting),
serialization libraries (T: Serialize + Deserialize),
iterator combinators (T: FnMut), concurrency primitives (T:
Send + Sync), and any generic API. Common bounds: Debug
(formatting), Clone (copying), PartialEq/Eq (equality),
PartialOrd/Ord (ordering), Default (default values),
Send/Sync (thread safety).

✅ TAKEAWAY
Trait bounds make generic code's requirements explicit in
function signatures and enable compile-time polymorphism
with zero runtime overhead through monomorphization. Start
with minimal bounds and add as needed - fewer bounds mean
more reusability, more bounds mean more capability, all
enforced at compile time for safety without sacrificing
performance."#,
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
            commentary: r#"📚 INTRODUCTION
Where clauses provide clearer syntax for complex trait
bounds by moving constraints after the function signature.
Instead of cluttering type parameters with bounds, you
separate "what types" from "what constraints," improving
readability for functions with multiple parameters or
complex bounds.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike inline bounds that make signatures unreadable with
complexity, where clauses maintain clarity as bounds grow.
This prevents: incomprehensible function signatures,
missing constraints in complex generics, and limitations of
inline syntax. Compared to inline bounds, where clauses
enable expressing constraints impossible otherwise: bounds
on concrete types (where i32: Display), lifetime bounds
(where T: 'a), and associated type constraints (where T:
Iterator<Item = i32>).

🔍 IMPORTANT DETAILS & INTRICACIES
The compiler treats where clauses identically to inline
bounds - zero performance difference, purely readability.
You can express complex relationships: where T: AsRef<U>,
bounds on constructed types (where Vec<T>: Clone), and
constraints that would be awkward inline. Convention: use
where for multiple parameters or multiple bounds per
parameter, inline for simple single bounds. Both compile to
the same monomorphized code.

💼 WHERE IT'S MOST USED
Trait definitions with complex associated function bounds,
generic APIs with many constraints, iterator combinators,
builder patterns requiring multiple bounds, and anywhere
signatures would become unreadable with inline syntax.
Essential in library design where clean, scannable
signatures improve API usability and documentation
generation.

✅ TAKEAWAY
Where clauses improve readability of complex generic
signatures without sacrificing any capability or
performance - they're pure syntax sugar that happens to
enable expressing constraints impossible with inline syntax
alone. Use them to keep function signatures focused on
logical structure while documenting type requirements
separately, making code more maintainable and
understandable."#,
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
            commentary: r#"📚 INTRODUCTION
Methods on generic types can introduce additional type
parameters beyond the struct's parameters. The mixup method
introduces U, allowing Point<T> to combine with Point<U> to
create Point<(T, U)>, demonstrating methods that are more
generic than their containing types.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike methods that only use the struct's type parameters,
method-level generics enable operations that combine or
transform types flexibly. This prevents: limiting
operations to the struct's original type parameters,
duplicating methods for different type combinations, and
runtime type conversions. Compared to fixed method
signatures, you can compose types in ways the struct
designer never anticipated, all type-safe and zero-cost.

🔍 IMPORTANT DETAILS & INTRICACIES
Type parameter scope is crucial: T comes from Point<T> and
exists in all impl blocks, while U exists only in mixup's
signature and body. Each call to mixup can use different U
types - the compiler monomorphizes for each unique (T, U)
combination actually used. If you call mixup with five
different types, five optimized versions are generated. No
runtime type checking, boxing, or indirection - just
specialized machine code.

💼 WHERE IT'S MOST USED
Standard library heavily uses this: Iterator::map<B, F>
introduces mapping function type, collect::<B> introduces
target collection type, parse::<F> introduces parsing
target. Common in builder APIs, conversion traits
(From/Into), combinators, and anywhere methods need to be
polymorphic beyond the struct's type parameters. Enables
fluent, composable APIs.

✅ TAKEAWAY
Method-level generic parameters let methods introduce type
flexibility beyond their struct's parameters, enabling
powerful composition and transformation operations while
maintaining compile-time type safety and zero runtime
overhead. This pattern is fundamental to Rust's composable,
zero-cost API design - methods can be as generic as needed
regardless of struct constraints."#,
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
            commentary: r#"📚 INTRODUCTION
Const generics parameterize types over compile-time
constant values (like array sizes) rather than types.
ArrayPair<T, N> is generic over both element type T and
array size N, making fixed-size arrays first-class in
generic programming without runtime overhead.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Before Rust 1.51 (const generics stabilization), you needed
separate implementations for each array size or used Vec
with runtime overhead. This prevents: code duplication for
different array sizes, runtime size checking and
allocation, and arrays being second-class to Vec in generic
code. Compared to languages without const generics, you can
write matrix libraries, SIMD code, and embedded systems
code that's generic over sizes yet fully optimized.

🔍 IMPORTANT DETAILS & INTRICACIES
N is a compile-time constant, not a runtime variable -
ArrayPair<i32, 3> and ArrayPair<i32, 5> are completely
distinct types. The compiler monomorphizes for each unique
(type, const) combination, embedding constant values
directly into generated code. This enables loop unrolling,
perfect memory layout optimization, and compile-time size
verification. You can use N in type expressions ([T; N])
and array initialization, but N must be known at compile
time. Currently supports integer and boolean const
parameters.

💼 WHERE IT'S MOST USED
Numerical computing: matrix libraries generic over
dimensions (Matrix<T, M, N>), SIMD intrinsics generic over
lane count, cryptography with fixed-size keys/blocks.
Embedded systems: fixed-size buffers without dynamic
allocation. Network protocols: packet parsers generic over
packet sizes. Any domain needing zero-cost fixed-size
abstractions with compile-time size guarantees.

✅ TAKEAWAY
Const generics extend Rust's generic programming to
compile-time constant values, enabling zero-cost
abstractions over array sizes and other constants with full
type-safety and optimization. The type system enforces size
compatibility at compile time while the compiler generates
perfectly optimized, specialized code - making fixed-size
data structures as flexible and safe as dynamic ones
without any runtime cost."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
