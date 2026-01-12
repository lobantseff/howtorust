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
            commentary: r#"Closures are anonymous functions that can capture variables from their surrounding environment, making them more powerful and flexible than regular functions. Using the lightweight |param| syntax instead of fn, closures enable inline definition of behavior without ceremony, perfect for short operations passed to higher-order functions like map and filter. The defining characteristic that distinguishes closures from functions is environment capture - a closure can "close over" variables from its enclosing scope, storing references or values for later use. This makes closures ideal for callbacks, iterators, and functional programming patterns where you need behavior parameterized by contextual data.

The technical implementation of closures leverages Rust's type inference system. While functions require explicit type annotations, closures infer parameter and return types from their usage context. When you write |x| x + 1, the compiler determines x's type from how the closure is called. Under the hood, each closure is compiled to a unique anonymous type that implements one or more of the Fn traits (Fn, FnMut, FnOnce), with captured variables stored as fields in this generated struct. This zero-cost abstraction means closures compile to code as efficient as handwritten structures with custom call methods, with no runtime overhead compared to manual implementation.

Rust's closure design balances the expressiveness of functional programming languages like JavaScript or Python with the zero-cost abstraction philosophy of systems programming. Unlike languages where closures always heap-allocate captured variables, Rust's ownership system allows closures to borrow data efficiently, moving to the heap only when necessary (e.g., when returned from functions). The type inference makes closures concise while maintaining full type safety - closures have concrete types known at compile time, not dynamic types like JavaScript's functions. This contrasts with C++'s lambdas which also have unique types but require more verbose capture specifications.

In practice, closures appear everywhere in idiomatic Rust code: iterator chains (numbers.iter().map(|x| x * 2)), threading (thread::spawn(|| println!("hello"))), option handling (value.map(|v| v + 1)), and callbacks in async code. The syntax is intentionally lightweight to encourage functional style. Start with simple closures for one-liners, using explicit type annotations when clarity demands it: |x: i32| -> i32 { x * 2 }. A common gotcha is that each closure has a unique type even if signatures match, so you can't store different closures in the same variable without trait objects or generics. Closures that don't capture anything can be coerced to function pointers (fn types) for C interop or when a concrete function type is required."#,
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
            commentary: r#"Closure type inference is one of Rust's most convenient features, allowing the compiler to deduce parameter and return types from usage context, eliminating the verbosity of explicit annotations while maintaining full type safety. When you write |x, y| x * y, the compiler examines how the closure is called - if you pass two i32 values, it infers both parameters as i32. This inference happens once, locking in concrete types for that specific closure. Once types are inferred from the first use, subsequent calls must match those types. You can override inference with explicit annotations when needed for clarity or when the compiler can't determine types from context alone, using the syntax |x: T| -> R { body }.

The technical mechanism relies on Rust's powerful type inference engine working backward from usage sites. The compiler generates a unique anonymous struct type for each closure, with fields for captured variables and type parameters for the closure's arguments. When you call the closure, the compiler unifies the argument types with the closure's parameter types, flowing type information bidirectionally. This is monomorphization - each closure instantiation generates specialized code for its inferred types. The critical insight is that inference locks in permanently: after |x| x + 1 is called with an i32, attempting to call it with an f64 causes a type error. The closure has one concrete type, not a polymorphic template.

Rust's inference design reflects the principle of local reasoning - types can be omitted when obvious from context, reducing syntactic noise without sacrificing safety. This contrasts with dynamically typed languages where closure parameter types are never checked, and with Java where lambda types must often be specified explicitly despite being inferrable. The approach is similar to C++ generic lambdas (auto parameters) but more rigorous - Rust's closures have concrete types after inference, while C++ lambdas with auto remain templates until instantiation. The gotcha is that even identical-looking closures have distinct types, making them incompatible for storage in the same variable or return from different branches.

In practice, rely on inference for most closures - it's concise and safe. Add explicit types when the compiler can't infer (e.g., empty closures, ambiguous contexts), for documentation in complex closures, or when storing closures in structs where the type becomes part of the API. The pattern of inference-then-lock means you can't use the same closure polymorphically over different types - create separate closures if needed. A subtle point: inference considers only the closure's usage, not its body, so |x| x + 1 could infer x as i32, u32, f64, etc., depending on how you call it. This can lead to surprising errors if you assume a particular type in the body but call with another."#,
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
            commentary: r#"Environment capture is the defining feature that makes closures "close over" their surrounding context, storing access to variables from the enclosing scope for use inside the closure body. When a closure references a variable from outside, it captures that variable automatically, making it available whenever the closure is called, even if the original scope has ended. This mechanism is what makes closures different from regular functions - functions can only access their parameters and globals, while closures can access any variable visible at their definition site. The name "closure" comes from this behavior of closing over the environment, creating a bundle of code plus captured context.

The technical implementation of capture follows Rust's ownership rules precisely. The compiler analyzes how the closure uses each captured variable and chooses the most restrictive capture mode necessary: immutable borrow (&T) if the variable is only read, mutable borrow (&mut T) if modified, or owned value (T) if moved. This automatic capture analysis generates a struct containing fields for each captured variable with the appropriate types. For example, a closure capturing x immutably and y mutably compiles to roughly: struct Closure<'a, 'b> { x: &'a i32, y: &'b mut i32 }. The closure's lifetime is constrained by the shortest lifetime of any borrowed reference, ensuring memory safety without garbage collection.

Rust's automatic capture mode selection reflects the principle of least privilege - closures take the minimum access necessary to function correctly, preventing unnecessary restrictions. This contrasts with C++ lambdas where you explicitly specify capture mode ([=], [&], [x, &y]), providing more control but requiring more thought. In languages like JavaScript or Python, closures always capture by reference with no ownership tracking, relying on garbage collection to prevent use-after-free. Rust's approach provides the convenience of automatic capture with the safety of compile-time borrow checking, catching errors like capturing references that outlive their data.

In practice, let the compiler infer capture modes automatically - it chooses correctly based on what you do with variables inside the closure. Closures that only read captured variables can be called multiple times and shared freely. Those that mutate captures need &mut self and can't be shared concurrently. Those that move captured values can only be called once. A common pattern is capturing configuration or context for callbacks: let threshold = 10; numbers.filter(|&x| x > threshold). Be aware that capturing creates dependencies - a closure borrowing x prevents mutation of x until the closure is dropped. For long-lived closures, consider using move to take ownership and avoid borrow checker conflicts."#,
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
            commentary: r#"The three closure traits - Fn, FnMut, and FnOnce - form a hierarchy that encodes how closures interact with their captured environment through the type system. FnOnce represents closures that consume their captured values and can be called at most once, since calling them moves ownership of captured data. FnMut represents closures that mutate their captures and require exclusive (&mut) access, callable multiple times but not concurrently. Fn represents closures that only read captures immutably, callable any number of times, even concurrently, making them the most flexible. The traits form an inheritance hierarchy: all closures implement FnOnce, FnMut closures also implement FnOnce, and Fn closures implement all three. This hierarchy enables generic code to specify the minimum capability needed.

The technical implementation maps closure capture modes to trait implementations through the compiler's analysis of the closure body. If a closure moves a captured variable (consuming ownership), it implements only FnOnce because calling it twice would use the moved value twice, which is impossible. If it mutates a capture (requiring &mut access), it implements FnMut and FnOnce. If it only reads captures immutably, it implements all three traits. The traits differ in their self parameter: Fn takes &self, FnMut takes &mut self, and FnOnce takes self (by value), reflecting the access level required. This encoding means accepting F: Fn in a generic function constrains callers to closures that don't mutate or consume captures, while F: FnOnce accepts any closure.

Rust's trait-based approach to closures solves the problem of expressing closure capabilities in the type system without hardcoded language rules. Unlike languages where you simply pass functions with no capability distinction, Rust's traits make the mutation and consumption explicit, enabling compiler-verified thread safety (Fn is Send+Sync if captures are) and preventing use-after-move bugs. This is more sophisticated than C++'s function objects where capabilities aren't encoded in types, requiring manual reasoning. The hierarchy design is elegant: code accepting FnOnce is most permissive (any closure works), Fn is most restrictive (only non-mutating closures), and FnMut sits in the middle.

In practice, when writing functions that accept closures, choose the least restrictive trait bound that satisfies your needs. Use FnOnce if you only call the closure once and don't care about mutations - this maximizes caller flexibility. Use FnMut if you call the closure multiple times and it may need to mutate state. Use Fn if you need to call the closure multiple times and want it to be thread-safe or callable from multiple places simultaneously. Iterator methods demonstrate this: map uses FnMut because it calls the closure multiple times, once per element. Be aware that requiring Fn unnecessarily restricts callers - they can't use closures that mutate, even if your use case would permit it. The traits are automatically implemented, so you never implement them manually."#,
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
            commentary: r#"The move keyword forces closures to take ownership of all captured variables rather than borrowing them, transferring the captured values into the closure's internal storage. This override of Rust's default capture-by-borrow behavior is essential when closures must outlive the scope where they're defined, particularly when passing closures to threads, returning them from functions, or storing them in data structures. Without move, closures borrow their environment, creating lifetime dependencies that prevent the closure from outliving its captured references. With move, the closure owns its data and can live independently, since ownership has been transferred from the outer scope to the closure itself.

The technical mechanism is straightforward: move instructs the compiler to transfer ownership of captured variables into the closure's generated struct, as if you wrote field: var instead of field: &var. For Copy types like integers, this means copying the value. For non-Copy types like String or Vec, this means moving ownership, making the variables unusable in the outer scope after the closure is created. The move keyword is all-or-nothing - it affects all captured variables, not individual ones. This changes the closure's trait implementations: a move closure capturing non-Copy types can only implement FnOnce since calling it consumes the owned values, while a move closure capturing only Copy types can still implement Fn.

Rust requires explicit move rather than making it automatic to prevent accidental ownership transfer, maintaining the principle of explicit control over resource ownership. This contrasts with languages like JavaScript where closures always capture by reference with garbage collection handling lifetimes, or C++ where you specify capture mode per variable ([=x] vs [&x]). Rust's move keyword provides a middle ground - automatic capture of all variables with explicit ownership semantics. The design ensures thread safety: threads require move closures because borrowed data from the spawning thread might be destroyed before the spawned thread finishes, creating dangling references. The compiler enforces this through Send trait bounds.

In practice, use move primarily in three scenarios: spawning threads (thread::spawn requires move to ensure thread safety), returning closures from functions (the closure must own its captures to outlive the function), and when you want to snapshot values at closure creation time rather than referencing them. A common pattern is thread::spawn(move || { use_x(x); }), transferring x to the new thread. Be aware that move prevents the outer scope from using moved variables afterward - clone values before the closure if you need them in both places. For Copy types, move has no observable difference from borrowing in terms of outer scope access. A subtle point: move closures can still borrow from the heap (e.g., capturing an Arc clones the Arc, not the underlying data)."#,
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
            commentary: r#"Accepting closures as function parameters enables powerful higher-order programming patterns where functions can be parameterized by behavior, not just data. The idiomatic approach uses generic type parameters with Fn trait bounds, written as <F> where F: Fn(Args) -> Return, meaning "F can be any type that implements the Fn trait with this specific signature." This allows the function to accept any closure (or function pointer) matching the specified signature, whether defined inline, stored in a variable, or passed through multiple layers. The generic approach is preferred over trait objects (Box<dyn Fn>) because it enables monomorphization - the compiler generates specialized versions for each concrete closure type, enabling inlining and zero-cost abstraction.

The technical implementation leverages Rust's generic system and trait bounds to create flexible yet type-safe APIs. When you write fn apply<F: Fn(i32) -> i32>(x: i32, f: F), the compiler generates a unique version of apply for each distinct closure type passed to it, specialized with the actual closure's implementation. This monomorphization means calling the closure has no dynamic dispatch overhead - it's as fast as a direct function call. The where clause syntax (where F: Fn(i32) -> i32) is often preferred for complex bounds as it improves readability. You can tighten the bound to FnMut or FnOnce based on your needs, controlling whether callers can pass closures that mutate or consume captures.

Rust's approach to closure parameters balances flexibility with performance, avoiding the overhead of function pointers or virtual dispatch when possible. Unlike languages with first-class functions that typically use dynamic dispatch for function parameters, Rust's generic approach provides compile-time polymorphism. This is similar to C++ templates but with trait bounds providing explicit constraints rather than duck typing. The design enables the zero-cost abstraction principle - abstractions like iterator.map(closure) compile to code as efficient as hand-written loops. The tradeoff is binary size: each closure type generates separate code, potentially increasing compilation time and executable size compared to dynamic dispatch, though the optimizer often eliminates duplicates.

In practice, use generic type parameters with Fn bounds when writing functions that accept closures, choosing the appropriate trait (Fn, FnMut, or FnOnce) based on how you'll call the closure. Prefer Fn for maximum caller flexibility when you don't need mutation. Use where clauses for complex bounds: fn process<F>(f: F) where F: Fn(&str) -> Result<i32, Error>. This pattern appears everywhere in standard library: Option::map, Iterator::filter, thread::spawn all accept closures this way. You can also accept both closures and function pointers by using the Fn trait, which function pointers implement. For stored closures or when monomorphization is undesirable, use trait objects: Box<dyn Fn(i32) -> i32>, trading performance for single code generation."#,
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
            commentary: r#"Returning closures from functions enables powerful patterns like function factories, partial application, and currying, where you create specialized functions programmatically based on runtime parameters. The impl Fn(Args) -> Return syntax allows returning closures without specifying their concrete type, which is compiler-generated and unnameable. The move keyword is essential because the returned closure must own all captured data - it outlives the function that created it, making borrowing impossible since the function's stack frame is destroyed on return. This combination creates closures that encapsulate both behavior and state, returning customized functions configured by the factory's parameters.

The technical challenge is that each closure has a unique, anonymous type generated by the compiler, making it impossible to write the return type explicitly. Prior to impl Trait, returning closures required Box<dyn Fn>, introducing heap allocation and dynamic dispatch overhead. The impl Trait syntax solves this by letting the compiler fill in the concrete type while keeping the closure on the stack or inline in the caller. Under the hood, the function returns a specific closure type implementing Fn, and impl Trait acts as an existential type hiding the concrete type from the caller. The move keyword ensures captured variables are transferred into the closure's storage, giving it independent ownership.

Rust's impl Trait design for return types reflects the evolution toward zero-cost abstractions without sacrificing usability. Earlier Rust required boxing all returned closures, imposing runtime overhead. impl Trait provides the ergonomics of dynamic dispatch (hiding the concrete type) with the performance of static dispatch (no runtime indirection). This is more powerful than languages with first-class functions like JavaScript or Python where all functions are heap-allocated references, and different from languages like Haskell where function types are explicit. The move requirement ensures memory safety - closures can't hold dangling references to the factory function's local variables.

In practice, use returning closures for function factories that create specialized behavior: authentication checkers with hardcoded credentials, validators with configured thresholds, formatters with captured locale settings. The pattern is powerful for APIs where you construct functions from configuration: fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 { move |x| x * n }. This enables partial application - fixing some arguments while leaving others open. Use move to capture configuration values by ownership, ensuring the closure is self-contained. Be aware that impl Trait return types are opaque - callers can't name the type, only use it through the trait. For returning different closures from different branches, you need trait objects (Box<dyn Fn>) since each closure has a unique type. The returned closures are particularly useful with iterator adapters and async runtimes."#,
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
            commentary: r#"Storing closures in structs creates powerful patterns like memoization, lazy evaluation, and strategy-based design where behavior is configured at construction time and invoked later. The struct holds both the closure (as a generic type parameter with a trait bound) and any associated state like cached results. This enables deferred computation - creating an expensive operation without executing it until needed, then caching results to avoid recomputation. The pattern demonstrates Rust's zero-cost abstraction: closures stored in structs compile to regular struct fields with static dispatch, incurring no runtime overhead compared to handwritten code, while providing elegant API design.

The technical implementation requires making the closure type a generic parameter of the struct since each closure has a unique compiler-generated type that cannot be named. The struct definition uses where clauses to bound the generic: struct Cacher<T> where T: Fn(u32) -> u32 { calculation: T, value: Option<u32> }. This stores the closure as a field alongside the cached value. Methods on the struct can call the closure through the Fn trait: (self.calculation)(arg), with parentheses disambiguating the field access from the call. The Option tracks whether the computation has run. The struct's lifetime is independent of the closure's captures (which must be owned if the struct is moved), making move closures common with this pattern.

Rust's ability to store closures in structs without heap allocation or virtual dispatch is a key differentiator from languages like Java (where you'd use anonymous classes or lambdas with interface types, introducing vtable dispatch) or dynamic languages (where functions are always reference types). This approach provides the flexibility of first-class functions with the performance of direct calls. The design enables sophisticated patterns from functional programming - lazy sequences, memoization, dependency injection - in a systems programming context. The tradeoff is complexity: generic type parameters and trait bounds are more verbose than simply storing a function reference.

In practice, use struct-stored closures for lazy evaluation (compute on first access), memoization (cache results), callbacks (store handlers for later invocation), and strategy pattern (configure behavior at construction). The example demonstrates a simple memoizer that caches one result; production code would use HashMap for multiple inputs. Common refinements include thread-safe caching with Mutex, eviction policies for bounded caches, and async closures for deferred async work. Be aware that this simple implementation has limitations: it caches only one input value and doesn't handle different arguments. Also, storing closures makes your struct generic over the closure type, which can complicate APIs - consider trait objects (Box<dyn Fn>) if the generic complexity is unwanted, trading static for dynamic dispatch."#,
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
            commentary: r#"Function composition creates a new function by combining two functions - the output of one becomes
the input of the next. This demonstrates advanced generic programming with closures, using three
type parameters to connect the input, intermediate, and output types.

This pattern is common in functional programming and enables building complex transformations from
simple building blocks. The compose function returns a closure that captures both input closures.

Note: Order matters - compose(f, g) means "do f first, then g" which reads as g(f(x))."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
