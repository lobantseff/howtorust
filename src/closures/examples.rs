// Closure examples organized in a separate file

use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "basic_closure",
            description: "Creating and calling a simple closure",
            code: r#"let add = |a, b| a + b;
let result = add(5, 3);
println!("5 + 3 = {}", result);

let greet = |name| format!("Hello, {}!", name);
println!("{}", greet("Alice"));"#,
            commentary: r#"📚 INTRODUCTION
Closures are anonymous functions that can capture variables from their surrounding environment. Unlike regular functions, closures use lightweight |param| syntax and can "close over" variables from their enclosing scope, making them ideal for inline operations, callbacks, and functional programming patterns.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to languages like JavaScript or Python where closures always heap-allocate captured variables, Rust's closures borrow data efficiently, moving to heap only when necessary. Solves: (1) Verbose function definitions for simple operations, (2) Need to pass context to callbacks without globals, (3) Iterator transformations requiring inline behavior, (4) Thread-safe capture of environment for concurrent code.

🔍 IMPORTANT DETAILS & INTRICACIES
Each closure compiles to a unique anonymous type implementing Fn traits (Fn, FnMut, FnOnce), with captured variables stored as struct fields. Type inference determines parameter and return types from usage context - when you write |x| x + 1, the compiler infers x's type from how the closure is called. This locks in concrete types on first use. Closures are zero-cost abstractions, compiling to code as efficient as handwritten structs with no runtime overhead. Each closure has a unique type even if signatures match, so different closures cannot be stored in the same variable without trait objects or generics. Closures capturing nothing can coerce to function pointers (fn types) for C interop.

💼 WHERE IT'S MOST USED
Iterator chains (numbers.iter().map(|x| x * 2)), threading (thread::spawn(|| println!("hello"))), option/result handling (value.map(|v| v + 1)), async callbacks, event handlers, functional transformations with filter/fold/reduce, custom sorting comparators, lazy evaluation patterns.

✅ TAKEAWAY
Closures are Rust's anonymous functions with environment capture, combining the expressiveness of functional programming with zero-cost abstraction. The lightweight |param| syntax enables inline behavior definition, while the compiler generates efficient code by creating unique types implementing Fn traits. Use closures for short operations passed to iterators, threads, and higher-order functions - they infer types from usage, capture only what's needed, and compile to code as fast as handwritten alternatives."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "closure_type_inference",
            description: "Type inference in closures",
            code: r#"// Types are inferred from usage
let multiply = |x, y| x * y;
println!("10 * 5 = {}", multiply(10, 5));

// Can also specify types explicitly
let divide = |x: f64, y: f64| -> f64 { x / y };
println!("10.0 / 3.0 = {}", divide(10.0, 3.0));"#,
            commentary: r#"📚 INTRODUCTION
Closure type inference allows the compiler to deduce parameter and return types from usage context, eliminating explicit type annotations while maintaining full type safety. Write |x, y| x * y and the compiler determines types from how you call the closure.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to Java where lambda types often need explicit specification, and unlike dynamically typed languages with no type checking, Rust infers types from context while maintaining safety. Solves: (1) Verbose type annotations for obvious closure parameters, (2) Syntactic noise in functional chains, (3) Repetitive type specifications in iterator methods, (4) Balance between conciseness and type safety.

🔍 IMPORTANT DETAILS & INTRICACIES
Type inference works backward from usage sites using bidirectional type flow. Once types are inferred from first use, they lock in permanently - after |x| x + 1 is called with i32, calling with f64 causes a type error. The closure has one concrete type, not a polymorphic template. Each closure generates specialized code through monomorphization. Inference considers only usage, not the body, so |x| x + 1 could infer x as i32, u32, or f64 depending on call site. You can override with explicit syntax: |x: i32| -> i32 { x * 2 }. This is more rigorous than C++ generic lambdas where auto parameters remain templates.

💼 WHERE IT'S MOST USED
Iterator chains where types are obvious from context, map/filter/fold operations, simple callbacks, inline transformations, option/result combinators, any functional programming pattern where verbosity would obscure intent.

✅ TAKEAWAY
Closure type inference eliminates redundant annotations while maintaining Rust's type safety guarantee by deducing types from usage context. The inference-then-lock mechanism gives each closure a concrete type after first use, preventing polymorphic usage but enabling zero-cost specialization. Rely on inference for concise code, adding explicit types only when the compiler can't infer (empty closures, ambiguous contexts) or for documentation in complex closures."#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "capturing_environment",
            description: "Closures capturing variables from their environment",
            code: r#"let x = 10;
let y = 20;

let add_to_x = |a| a + x;
let add_both = |a| a + x + y;

println!("5 + x = {}", add_to_x(5));
println!("5 + x + y = {}", add_both(5));"#,
            commentary: r#"📚 INTRODUCTION
Environment capture is the defining feature where closures "close over" their surrounding context, automatically storing access to variables from the enclosing scope. Unlike regular functions that only access parameters and globals, closures can reference any variable visible at their definition site.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to C++ lambdas requiring explicit capture specifications ([=], [&]), Rust automatically chooses capture mode. Unlike JavaScript/Python where closures always capture by reference with garbage collection, Rust uses compile-time borrow checking. Solves: (1) Passing context to callbacks without global variables, (2) Creating specialized functions with captured configuration, (3) Iterator filters with local thresholds, (4) Preventing use-after-free when captured data outlives closure.

🔍 IMPORTANT DETAILS & INTRICACIES
The compiler analyzes usage and chooses the most restrictive capture mode necessary: immutable borrow (&T) for reads, mutable borrow (&mut T) for modifications, or owned value (T) for moves. This generates a struct with fields for captured variables - a closure capturing x immutably and y mutably compiles to roughly struct Closure<'a, 'b> { x: &'a i32, y: &'b mut i32 }. The closure's lifetime is constrained by the shortest lifetime of borrowed references. Capturing creates dependencies - a closure borrowing x prevents mutation of x until the closure drops. This automatic capture mode selection follows the principle of least privilege.

💼 WHERE IT'S MOST USED
Iterator filters with local thresholds (numbers.filter(|&x| x > threshold)), callbacks capturing configuration state, event handlers with context, thread closures accessing parent scope data, partial application patterns, currying implementations.

✅ TAKEAWAY
Environment capture enables closures to bundle code with context by automatically capturing variables from their surrounding scope using the minimum access necessary. The compiler analyzes usage to choose between immutable borrow, mutable borrow, or ownership transfer, generating efficient structs with captured data as fields. Let the compiler infer capture modes automatically - closures reading captures can be shared, those mutating need exclusive access, and those moving values can only be called once."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "fn_traits",
            description: "Three closure traits: Fn, FnMut, FnOnce",
            code: r#"// Fn: borrows immutably
let x = vec![1, 2, 3];
let print_vec = || println!("Vec: {:?}", x);
print_vec();
print_vec();  // Can call multiple times

// FnMut: borrows mutably
let mut count = 0;
let mut increment = || {
    count += 1;
    println!("Count: {}", count);
};
increment();
increment();

// FnOnce: takes ownership
let s = String::from("hello");
let consume = || {
    let _owned = s;
    println!("Consumed string");
};
consume();
// consume();  // Error: can only call once!"#,
            commentary: r#"📚 INTRODUCTION
The three closure traits - Fn, FnMut, and FnOnce - form a hierarchy encoding how closures interact with captured environment through the type system. Fn borrows immutably (callable any number of times), FnMut borrows mutably (callable multiple times but exclusively), and FnOnce consumes captures (callable at most once).

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike languages where functions have no capability distinction, Rust's traits make mutation and consumption explicit in types. Compared to C++ function objects where capabilities aren't type-encoded, requiring manual reasoning. Solves: (1) Compiler-verified thread safety (Fn is Send+Sync if captures are), (2) Preventing use-after-move bugs when closures consume values, (3) Expressing whether closures can be called repeatedly or exclusively, (4) Enabling generic code to specify minimum capability needed.

🔍 IMPORTANT DETAILS & INTRICACIES
The traits differ in self parameter: Fn takes &self, FnMut takes &mut self, FnOnce takes self (by value). This forms an inheritance hierarchy - all closures implement FnOnce, FnMut closures also implement FnOnce, and Fn closures implement all three. The compiler automatically determines implementation based on closure body: moving captured variables means only FnOnce, mutating captures means FnMut + FnOnce, only reading means all three traits. Accepting F: Fn constrains callers to non-mutating closures, while F: FnOnce accepts any closure. Iterator.map uses FnMut because it calls the closure multiple times. These traits are auto-implemented, never manual.

💼 WHERE IT'S MOST USED
Generic functions accepting closures (choosing appropriate bound), iterator methods (map uses FnMut, once uses FnOnce), thread spawn (requires FnOnce + Send), Option::map and similar combinators, callback systems, any API that needs to express closure call semantics in types.

✅ TAKEAWAY
The Fn trait hierarchy encodes closure capabilities in the type system, with Fn (immutable borrows), FnMut (mutable borrows), and FnOnce (consumes values) forming a hierarchy where Fn implements all three and FnOnce is the most permissive. Choose trait bounds based on usage: FnOnce for single-call flexibility, FnMut for repeated calls with possible mutation, Fn for thread-safe multi-call scenarios. The compiler automatically implements the appropriate traits based on what the closure does with captured variables."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "move_keyword",
            description: "Using move to force ownership transfer",
            code: r#"let x = vec![1, 2, 3];

// move forces closure to take ownership
let consume = move || {
    println!("Vec: {:?}", x);
};

consume();
// println!("{:?}", x);  // Error: x was moved

// Useful for threads
let y = String::from("Hello");
let closure = move || {
    println!("{}", y);
};
closure();"#,
            commentary: r#"📚 INTRODUCTION
The move keyword forces closures to take ownership of all captured variables rather than borrowing them, transferring values into the closure's internal storage. This overrides Rust's default capture-by-borrow behavior when closures need to outlive their defining scope.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to JavaScript where closures always capture by reference with garbage collection, or C++ where you specify per-variable capture mode ([=x] vs [&x]), Rust provides explicit all-or-nothing ownership transfer. Solves: (1) Thread safety - preventing borrowed data from being destroyed before spawned thread finishes, (2) Returning closures from functions that must own captures to outlive the function, (3) Storing closures in data structures with independent lifetimes, (4) Snapshotting values at closure creation time rather than referencing them.

🔍 IMPORTANT DETAILS & INTRICACIES
The move keyword transfers ownership of captured variables into the closure's generated struct (field: var instead of field: &var). For Copy types like integers, this copies the value. For non-Copy types like String or Vec, this moves ownership, making variables unusable in outer scope after closure creation. The keyword is all-or-nothing, affecting all captures. A move closure capturing non-Copy types can only implement FnOnce since calling consumes owned values, but move closures with only Copy types can still implement Fn. Thread::spawn requires move closures enforced through Send trait bounds. Move closures can still borrow from heap - capturing Arc clones the Arc pointer, not underlying data.

💼 WHERE IT'S MOST USED
Thread spawning (thread::spawn(move || {...})), returning closures from functions, storing closures in structs/enums with independent lifetimes, async tasks requiring 'static lifetime, capturing values for callbacks that outlive current scope, snapshotting configuration state.

✅ TAKEAWAY
The move keyword forces closures to take ownership of all captured variables, essential for closures that must outlive their defining scope such as in threads, returned values, or stored data structures. Use move in three primary scenarios: spawning threads where borrowed data might be destroyed too early, returning closures that must own their captures, and snapshotting values at creation time. Be aware that move prevents outer scope from using moved non-Copy variables afterward, though Copy types remain accessible."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "closures_as_parameters",
            description: "Passing closures as function parameters",
            code: r#"fn apply_operation<F>(x: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

let double = |n| n * 2;
let square = |n| n * n;

println!("Double 5: {}", apply_operation(5, double));
println!("Square 5: {}", apply_operation(5, square));

// Can also use inline closures
println!("Add 10: {}", apply_operation(5, |n| n + 10));"#,
            commentary: r#"📚 INTRODUCTION
Accepting closures as function parameters enables higher-order programming where functions are parameterized by behavior, not just data. The idiomatic approach uses generic type parameters with Fn trait bounds: <F> where F: Fn(Args) -> Return, accepting any closure matching the signature.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike languages with first-class functions using dynamic dispatch, Rust's generic approach provides compile-time polymorphism. Similar to C++ templates but with explicit trait constraints instead of duck typing. Solves: (1) Zero-cost abstractions - iterator.map(closure) compiles as efficiently as hand-written loops, (2) Type-safe behavior parameterization, (3) Inlining opportunities eliminated by function pointers, (4) Accepting both closures and function pointers through unified trait interface.

🔍 IMPORTANT DETAILS & INTRICACIES
Monomorphization generates a unique version of the function for each distinct closure type passed, specializing with the actual implementation. This means no dynamic dispatch overhead - calling the closure is as fast as direct function calls. The where clause syntax (where F: Fn(i32) -> i32) improves readability for complex bounds. Choose Fn, FnMut, or FnOnce based on usage to control caller flexibility. Generic approach is preferred over trait objects (Box<dyn Fn>) for performance but increases binary size since each closure type generates separate code. Function pointers implement Fn trait, allowing them to be passed too.

💼 WHERE IT'S MOST USED
Iterator methods (map, filter, fold accepting closures), Option/Result combinators (and_then, map), custom higher-order functions, sorting with custom comparators, configuration callbacks, event handlers, any API parameterized by behavior. Pervasive in functional-style Rust code.

✅ TAKEAWAY
Accept closures as parameters using generic type parameters with Fn trait bounds to enable zero-cost higher-order programming through monomorphization. Choose the appropriate trait (Fn, FnMut, or FnOnce) based on how you'll use the closure - Fn for maximum caller flexibility, FnMut for repeated calls with mutation, FnOnce for single-use consumption. This pattern compiles to code as efficient as hand-written alternatives while providing elegant functional abstractions."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "returning_closures",
            description: "Returning closures from functions",
            code: r#"fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

let add_5 = make_adder(5);
let add_10 = make_adder(10);

println!("7 + 5 = {}", add_5(7));
println!("7 + 10 = {}", add_10(7));"#,
            commentary: r#"📚 INTRODUCTION
Returning closures from functions enables function factories, partial application, and currying patterns where you create specialized functions programmatically based on runtime parameters. The impl Fn(Args) -> Return syntax returns closures without naming their compiler-generated type, while move ensures the closure owns captured data.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to languages like JavaScript/Python where functions are heap-allocated references, Rust's impl Trait provides zero-cost stack-based returns. Earlier Rust required Box<dyn Fn> with heap allocation overhead. Solves: (1) Creating specialized functions from configuration without globals, (2) Partial application - fixing some arguments while leaving others open, (3) Memory safety by preventing dangling references to factory function's locals, (4) Zero-cost abstraction avoiding heap allocation and dynamic dispatch.

🔍 IMPORTANT DETAILS & INTRICACIES
Each closure has unique, anonymous compiler-generated type impossible to write explicitly. impl Trait acts as existential type hiding concrete type while keeping closure on stack or inline. The move keyword is essential - returned closure must own captures since it outlives the function whose stack frame is destroyed on return. impl Trait provides ergonomics of dynamic dispatch (hidden type) with performance of static dispatch (no indirection). Return types are opaque - callers can't name the type, only use through trait. Returning different closures from different branches requires trait objects (Box<dyn Fn>) since each has unique type.

💼 WHERE IT'S MOST USED
Function factories creating specialized behavior (make_multiplier, make_validator), authentication checkers with captured credentials, validators with configured thresholds, formatters with locale settings, partial application patterns, configuration-based function construction, iterator adapter chains, async runtime task builders.

✅ TAKEAWAY
Return closures from functions using impl Fn syntax to create function factories that produce specialized behavior from runtime configuration, enabling partial application and currying patterns with zero-cost abstraction. Always use move keyword since returned closures must own captured data to outlive the factory function's stack frame. This pattern provides the convenience of dynamically created functions while maintaining Rust's performance guarantees through stack allocation and static dispatch."#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "closure_caching",
            description: "Caching expensive computations with closures",
            code: r#"struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

let mut expensive = Cacher::new(|x| {
    println!("Calculating...");
    x * 2
});

println!("First call: {}", expensive.value(10));
println!("Second call: {}", expensive.value(10));  // Uses cached"#,
            commentary: r#"📚 INTRODUCTION
Storing closures in structs creates patterns like memoization, lazy evaluation, and strategy-based design where behavior is configured at construction and invoked later. The struct holds the closure as a generic type parameter with trait bounds, plus any associated state like cached results.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike Java (anonymous classes with vtable dispatch) or dynamic languages (function references), Rust stores closures without heap allocation or virtual dispatch. Solves: (1) Deferred computation - create expensive operations without executing until needed, (2) Memoization avoiding repeated calculations, (3) Strategy pattern configuring behavior at construction, (4) Lazy sequences with on-demand evaluation, (5) Callback storage for later invocation.

🔍 IMPORTANT DETAILS & INTRICACIES
Each closure has unique compiler-generated type that cannot be named, requiring generic type parameter: struct Cacher<T> where T: Fn(u32) -> u32. Call closure through trait: (self.calculation)(arg), with parentheses disambiguating field access from call. Closures compile to regular struct fields with static dispatch, zero runtime overhead. The struct's lifetime is independent of closure's captures which must be owned if struct moves, making move closures common. Storing closures makes struct generic over closure type, complicating APIs - use trait objects (Box<dyn Fn>) to trade static for dynamic dispatch if needed.

💼 WHERE IT'S MOST USED
Lazy evaluation (compute on first access), memoization caches, callback systems storing handlers, strategy pattern implementations, dependency injection, iterator adapters with custom behavior, state machines with configurable transitions, configuration-driven processing.

✅ TAKEAWAY
Store closures in structs using generic type parameters with Fn trait bounds to enable memoization, lazy evaluation, and strategy patterns with zero-cost abstraction. This pattern compiles closures to regular struct fields with static dispatch, providing the flexibility of first-class functions with the performance of direct calls. Production implementations often add HashMap for multiple inputs, Mutex for thread safety, and eviction policies for bounded caches."#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "closure_composition",
            description: "Composing multiple closures",
            code: r#"fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

let add_one = |x: i32| x + 1;
let double = |x: i32| x * 2;

let add_then_double = compose(add_one, double);
let double_then_add = compose(double, add_one);

println!("(5 + 1) * 2 = {}", add_then_double(5));
println!("(5 * 2) + 1 = {}", double_then_add(5));"#,
            commentary: r#"📚 INTRODUCTION
Function composition creates a new function by combining two functions where the output of one becomes the input of the next. Using three type parameters (A, B, C), this demonstrates advanced generic programming with closures, connecting input, intermediate, and output types.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Common in functional programming languages like Haskell with built-in composition operators, less common in imperative languages. Solves: (1) Building complex transformations from simple building blocks, (2) Reusability by combining existing functions without rewriting logic, (3) Pipeline construction for data transformations, (4) Expressing computation as function chains rather than procedural steps.

🔍 IMPORTANT DETAILS & INTRICACIES
The compose function uses three type parameters: F: Fn(A) -> B and G: Fn(B) -> C, returning impl Fn(A) -> C. Returns a closure that captures both input closures with move, then applies them in sequence: move |x| g(f(x)). Order matters - compose(f, g) means "do f first, then g". The returned closure owns both captured closures, making it self-contained. Each composed closure has unique type, so storing multiple compositions requires trait objects or generics. Type inference flows through the chain - intermediate type B is inferred from connecting f's output to g's input.

💼 WHERE IT'S MOST USED
Data transformation pipelines, iterator adapter chains, middleware composition in web frameworks, parser combinators, validation chains, functional programming patterns, building complex predicates from simple ones, stream processing workflows.

✅ TAKEAWAY
Function composition combines two functions into one by connecting output to input, enabling complex transformations from simple building blocks using advanced generics with three type parameters. The pattern captures both closures with move and returns a new closure applying them in sequence. This functional programming technique builds reusable transformation pipelines, expressing computation as declarative function chains rather than imperative procedural steps."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
