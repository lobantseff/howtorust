// Iterators: Processing sequences of values

use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "basic_iteration",
            description: "Basic iteration with for loops",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

for num in numbers.iter() {
    println!("Number: {}", num);
}

// Can also iterate over ranges
for i in 0..5 {
    println!("Index: {}", i);
}"#,
            commentary: r#"Iterators provide a unified, efficient abstraction for processing sequences of values one at a time, whether those sequences come from collections, ranges, files, or custom sources. The iter() method creates an iterator that borrows each element immutably, allowing you to process collection contents without taking ownership or modifying them. For loops are syntactic sugar over iterators - when you write "for x in collection", Rust desugars this to creating an iterator and repeatedly calling next() until None is returned. Ranges like 0..5 are themselves iterators, demonstrating that iteration isn't tied to collections - any type implementing Iterator can be used in for loops or with iterator combinators.

The technical foundation is the Iterator trait, which requires implementing a single method: next(&mut self) -> Option<Self::Item>. This method advances the iterator and returns Some(value) for the next element or None when exhausted. The &mut self parameter means iterators track internal state (current position) and mutate it on each next() call. The Option return type elegantly handles the end-of-sequence condition without special sentinel values or exceptions. Collections provide multiple iterator methods: iter() for borrowing, iter_mut() for mutable borrowing, and into_iter() for consuming ownership. This flexibility lets you choose the appropriate access pattern based on whether you need to read, modify, or consume the collection.

Rust's iterator design reflects a commitment to zero-cost abstractions - iterators compile to code as efficient as hand-written indexed loops, often better due to optimizer opportunities. This contrasts with languages like Python where iteration involves runtime protocol dispatch and boxed values, or Java where iterators allocate objects. Rust's iterators are statically typed with known sizes, enabling inline iteration with no indirection. The design also unifies different iteration patterns under one trait: external iteration (you pull values) as in for loops, internal iteration (you provide a closure) as in for_each, and lazy iteration through adapters. This is more powerful than C++'s STL iterators which require separate begin/end pairs and pointer arithmetic.

In practice, use iter() when you need to read collection elements without consuming the collection - this is the most common case. Use for loops for straightforward sequential processing. Ranges are convenient for numeric iteration and integrating with array indexing. The iter() family returns different iterator types: slice::Iter<T> for vectors, std::collections::hash_map::Iter<K, V> for hashmaps, etc., but all implement the Iterator trait uniformly. A key insight is that iterators are lazy - they do nothing until consumed by a loop or terminal operation. Be aware that calling iter() multiple times creates independent iterators with separate state, allowing multiple concurrent traversals of the same data."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "iterator_methods",
            description: "Common iterator methods: map, filter, collect",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
println!("Doubled: {:?}", doubled);

let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).copied().collect();
println!("Evens: {:?}", evens);"#,
            commentary: r#"Iterator adapters like map and filter are the building blocks of functional-style data transformation in Rust, providing a declarative way to describe data processing pipelines. The map adapter transforms each element by applying a function, while filter selectively passes through elements matching a predicate. Crucially, these adapters are lazy - they don't execute immediately but instead return new iterator types that will perform the transformation when consumed. This laziness enables powerful optimizations and composability. The collect() consumer triggers evaluation by pulling all values through the iterator chain and gathering them into a collection, with the target collection type inferred from context or specified via the turbofish syntax collect::<Vec<_>>().

The technical implementation demonstrates Rust's zero-cost abstraction principle. Each adapter returns a new iterator type wrapping the previous iterator: map returns Map<I, F>, filter returns Filter<I, P>. These wrappers implement Iterator by delegating to the inner iterator's next() and applying their transformation. For example, Map::next() calls the wrapped iterator's next(), applies the mapping function to the result, and returns it. The compiler can see through these layers, often optimizing the entire chain into a single loop with no intermediate allocations. The collect() method uses the FromIterator trait to convert the iterator into various collection types, relying on type inference to determine the target.

Rust's iterator adapter design brings functional programming's expressiveness to systems programming without the traditional performance penalty. Unlike languages like Python where map and filter create intermediate lists (pre-Python 3) or lazy objects with runtime overhead, Rust's adapters compile to direct iteration with inlined transformations. This is similar to Java's Stream API but without boxing or virtual calls. The design encourages declarative code that's often clearer than imperative loops - "numbers.iter().map(|x| x * 2).filter(|x| x > 10)" directly expresses what you want rather than how to get it. The compiler's optimizer can also vectorize iterator chains, using SIMD instructions where appropriate.

In practice, use iterator adapters to build readable data transformation pipelines. The map adapter is essential for transformations: converting types, extracting fields, applying functions. The filter adapter selects elements: removing nulls, finding matches, validating conditions. Chain multiple adapters to build complex transformations: iter().filter(predicate).map(transform).collect(). Remember that adapters are lazy - you must consume the iterator with collect(), for_each(), sum(), or similar terminal operations to trigger execution. The copied() or cloned() adapters are often needed after filter to convert &T to T. Be mindful of the difference between iter() which yields &T and into_iter() which yields T - this affects what your closures receive and whether you need dereference operators."#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "consuming_adapters",
            description: "Consuming adapters: sum, collect, fold",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

let sum: i32 = numbers.iter().sum();
println!("Sum: {}", sum);

let product: i32 = numbers.iter().product();
println!("Product: {}", product);

let sum_doubled: i32 = numbers.iter().map(|x| x * 2).sum();
println!("Sum of doubled: {}", sum_doubled);"#,
            commentary: r#"Consuming adapters are terminal operations that consume the iterator by taking ownership and pulling all values through to produce a final result, in contrast to lazy adapters which simply wrap the iterator. Methods like sum(), product(), count(), and collect() immediately execute the entire iterator chain, aggregating values into a single output. The "consuming" terminology reflects that these methods take self (by value) rather than &self or &mut self, moving ownership of the iterator and making it unusable afterward. This ownership transfer is essential because the iterator is fully exhausted - after sum() runs through all elements, there's nothing left to iterate over.

The technical mechanism relies on trait implementations for specific operations. The sum() and product() methods require that the iterator's Item type implements the Sum or Product trait respectively, ensuring only summable/multipliable types can use these operations. These methods work by calling next() repeatedly, accumulating results through addition or multiplication. The type inference system determines the output type from context - sum::<i32>() explicitly specifies the result type when inference can't determine it. You can chain lazy adapters before consuming: numbers.iter().map(|x| x * 2).sum() efficiently computes the sum of doubled values in a single pass with no intermediate allocations, as the compiler fuses the transformations.

Rust's design of consuming vs. non-consuming operations makes resource usage explicit in the type system. By taking self instead of &self, consuming adapters signal through the API that the iterator is exhausted and unusable afterward. This prevents bugs where you attempt to use an exhausted iterator, which in languages without move semantics might return incorrect results or panic. The design also enables powerful optimizations - since the compiler knows the iterator is consumed in one operation, it can generate tight loops without worrying about iterator state preservation. This contrasts with languages where iterators are references and consumption isn't tracked, requiring runtime checks for exhaustion.

In practice, use consuming adapters to reduce iterator results to single values: sum() for totals, product() for factorials or combined multiplications, count() for size, max()/min() for extremes, collect() for gathering into collections. These are terminal operations - place them at the end of iterator chains. The pattern of chaining lazy adapters followed by a consumer is fundamental: numbers.iter().filter(condition).map(transform).sum() processes elements through the pipeline and computes the final result in one efficient pass. Be aware that sum() and product() require explicit type annotation if the compiler can't infer the result type: let total: i32 = iterator.sum(). Also note that these methods consume the iterator by value, so you can't reuse it - clone the original collection if you need multiple passes."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "chaining_iterators",
            description: "Chaining multiple iterator adapters",
            code: r#"let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: Vec<i32> = numbers
    .iter()
    .filter(|x| *x % 2 == 0)  // Keep evens
    .map(|x| x * x)            // Square them
    .filter(|x| *x > 10)       // Keep those > 10
    .collect();

println!("Result: {:?}", result);"#,
            commentary: r#"Iterator adapter chaining is the cornerstone of functional programming in Rust, enabling complex data transformations to be expressed as readable, composable pipelines. Each adapter method returns a new iterator type, allowing fluent method chaining where each step builds on the previous: iter().filter().map().filter() creates a layered transformation pipeline. The entire chain remains lazy - no computation occurs until a consuming operation like collect(), sum(), or for_each() triggers evaluation, at which point values flow through all transformation layers in a single pass. This combines the clarity of declarative code with the efficiency of imperative loops, often achieving both simultaneously.

The technical implementation leverages Rust's type system to nest iterator wrappers. When you write iter().filter(f).map(g), the compiler generates approximately Filter<Iter, F> wrapped in Map<Filter<Iter, F>, G>. Each wrapper implements Iterator by delegating to its inner iterator, applying its transformation, and passing the result forward. The beauty is that the compiler can see through all these layers during optimization, often inlining everything into a single efficient loop. No intermediate collections are allocated - values flow through the pipeline one at a time, with each transformation applied inline. The type system ensures type safety at each step: if map produces Iterator<Item=T>, the next filter's closure receives &T.

Rust's chaining design brings the elegance of Haskell or Scala's collection pipelines to systems programming without garbage collection or runtime overhead. Unlike object-oriented languages where method chaining often involves creating intermediate collections (older Java, most of Ruby), Rust's lazy evaluation ensures zero intermediate allocations. This is similar to C++20 ranges but with a more mature, ergonomic API. The design philosophy is that complex transformations should be expressible as linear reading sequences that mirror how you'd describe the process in English: "take numbers, keep even ones, square them, keep those over 10." This is often clearer than nested loops where the logic is spread across multiple indentation levels and loop indices.

In practice, build transformation pipelines by chaining adapters in logical order: source.iter().filter(stage1).map(transform).filter(stage2).collect(). Each step should perform one logical operation, making the pipeline self-documenting. Long chains remain readable because each method clearly states its purpose. The compiler optimizes chains aggressively, so prefer clarity over premature optimization. A common pattern is filtering before mapping to reduce wasted transformations: filter first, then transform the survivors. Be mindful of ownership: iter() yields &T, so your closures work with references, while into_iter() yields T. Use copied() or cloned() adapters when you need owned values. For debugging, insert inspect(|x| println!("{:?}", x)) to observe values flowing through the pipeline at any point."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "fold_reduce",
            description: "Using fold and reduce for accumulation",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

// fold: provide initial value
let sum = numbers.iter().fold(0, |acc, x| acc + x);
println!("Sum with fold: {}", sum);

let factorial = (1..=5).fold(1, |acc, x| acc * x);
println!("5! = {}", factorial);

// reduce: no initial value, returns Option
let max = numbers.iter().reduce(|a, b| if a > b { a } else { b });
println!("Max: {:?}", max);"#,
            commentary: r#"The fold and reduce methods are fundamental iterator operations that accumulate values by repeatedly applying a binary function, enabling general-purpose aggregation and computation. fold(initial, closure) takes an explicit initial accumulator value and a function that combines the accumulator with each element, threading state through the iteration to produce a final result. reduce(closure) is similar but uses the first element as the initial accumulator, returning Option<T> to handle empty iterators gracefully. fold is remarkably versatile - you can implement virtually any consuming operation (sum, product, collect, count, max) using fold, making it the "universal aggregator." The closure receives (accumulator, element) and returns the new accumulator value.

The technical implementation is a simple loop that maintains accumulator state, calling the closure for each element with the current accumulator and the element, then updating the accumulator with the result. For fold, the initial value is provided explicitly; for reduce, the first element is used as the initial accumulator and iteration begins with the second element. The critical difference is that fold always returns T (the accumulator type), while reduce returns Option<T> to handle the case where the iterator is empty and no first element exists. Both methods consume the iterator completely, calling next() until None. The accumulator can have a different type than the iterator's items, enabling transformations like collecting into different data structures.

Rust's fold design follows the functional programming tradition of catamorphisms - recursive data structure consumption that produces a single value. The concept originates from languages like Haskell where fold (called foldl/foldr) is fundamental. Rust makes fold more accessible through method syntax and explicit initial values rather than Haskell's implicit patterns. Unlike imperative accumulation with mutable variables in a loop, fold encapsulates the accumulation pattern, making it more composable and amenable to optimization. The design is similar to JavaScript's reduce but with Rust's ownership semantics requiring careful thought about whether the accumulator moves or borrows. reduce was added later to handle the common case where you don't have a natural initial value, avoiding boilerplate like fold(iter.next().unwrap(), ...).

In practice, use fold for general-purpose accumulation when specialized methods (sum, collect) don't fit. Common use cases include calculating running totals with additional logic, building custom data structures, implementing stateful transformations, and combining elements in domain-specific ways. For example, fold is perfect for concatenating strings with separators or accumulating into complex types. Use reduce when you want to combine elements pairwise without an external initial value, like finding the maximum or concatenating strings. Be aware that fold requires choosing an initial value carefully - for multiplication use 1, for addition use 0, for building collections use the empty collection. The accumulator type can differ from the element type, enabling powerful transformations: numbers.fold(Vec::new(), |mut acc, x| { acc.push(x * 2); acc })."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "enumerate_zip",
            description: "Using enumerate and zip",
            code: r#"let names = vec!["Alice", "Bob", "Charlie"];

// enumerate: add indices
for (i, name) in names.iter().enumerate() {
    println!("{}: {}", i, name);
}

// zip: combine two iterators
let ages = vec![25, 30, 35];
let combined: Vec<_> = names.iter().zip(ages.iter()).collect();
println!("Combined: {:?}", combined);"#,
            commentary: r#"The enumerate and zip adapters are essential for coordinating iteration across multiple data sources or when you need positional information alongside values. enumerate() wraps an iterator to yield (usize, T) tuples where the usize is a zero-based index and T is the original element, providing position information that's often needed for indexing, logging, or conditional logic based on position. zip() combines two iterators into a single iterator yielding (A, B) tuples containing corresponding elements from both sources. The zipped iterator stops when either input iterator is exhausted, preventing index-out-of-bounds errors and naturally handling sequences of different lengths. These adapters enable patterns that would require manual index tracking or parallel iteration in imperative code.

The technical implementation of enumerate maintains an internal counter initialized to zero, incrementing it with each next() call and pairing it with the element from the wrapped iterator. For zip, the implementation holds both iterators and calls next() on each, combining the results if both return Some or returning None if either is exhausted. This early termination on the shorter sequence is a deliberate design choice - it prevents accessing undefined elements and makes zip safe with sequences of unknown or mismatched lengths. The returned tuples can be destructured in closures or for loops: for (i, val) in iter.enumerate() or for (a, b) in iter1.zip(iter2), making the syntax clean despite the added complexity.

Rust's enumerate and zip designs solve common iteration problems with zero-cost abstractions. enumerate eliminates the error-prone pattern of manually maintaining index variables (let mut i = 0; for x in iter { ... i += 1; }), which can lead to off-by-one errors or forgotten increments. zip eliminates parallel iteration with manual indexing (for i in 0..n { use arr1[i] and arr2[i] }), which is unsafe if lengths differ. These abstractions compile to efficient code - enumerate becomes a simple counter increment, zip becomes dual next() calls. This is more elegant than C++'s approach of explicit index loops or Python's enumerate/zip which create tuple objects at runtime. Rust's version has no runtime overhead and provides compile-time type safety for the tuple elements.

In practice, use enumerate when you need element positions for indexing other structures, reporting progress, conditional logic based on position, or inserting position-aware separators (like commas between elements). Use zip when processing parallel arrays (coordinates from separate x/y arrays), combining keys with values, comparing sequences element-wise, or any scenario where you need corresponding elements from multiple sources. Common patterns include pairing names with ages, combining headers with data rows, or element-wise arithmetic on vectors. Be aware that zip truncates to the shorter iterator - this is usually correct, but if you need the longer sequence, consider using itertools::zip_longest. Also, enumerate indices start at zero following Rust's indexing convention."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "find_any_all",
            description: "Finding elements: find, any, all",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

let first_even = numbers.iter().find(|x| *x % 2 == 0);
println!("First even: {:?}", first_even);

let has_even = numbers.iter().any(|x| *x % 2 == 0);
println!("Has even: {}", has_even);

let all_positive = numbers.iter().all(|x| *x > 0);
println!("All positive: {}", all_positive);"#,
            commentary: r#"The find, any, and all methods provide efficient, expressive ways to search iterators and test predicates across elements, with built-in short-circuit evaluation that stops iteration as soon as the answer is determined. find(predicate) searches for the first element satisfying the predicate, returning Option<T> - Some with the found element or None if no match exists. any(predicate) returns bool indicating whether at least one element matches, useful for existence checks. all(predicate) returns bool indicating whether every element matches, useful for validation. The short-circuit behavior is critical for performance: any stops at the first true, all stops at the first false, and find stops at the first match, avoiding unnecessary iteration over remaining elements.

The technical implementation of these methods is straightforward but demonstrates Rust's efficiency. Each method calls next() repeatedly, testing the predicate on each element. find returns immediately upon finding a matching element (Some(element)) or None if the iterator is exhausted. any returns true on the first element where predicate returns true, or false if none match. all returns false on the first element where predicate returns false, or true if all match. This short-circuiting is built into the methods - you don't need to manually break from loops. The predicate closures receive references to elements (&T) for find when using iter(), allowing inspection without consuming the collection, though find consumes the iterator itself.

Rust's search methods bring functional programming expressiveness to iteration with optimal performance. The short-circuit behavior matches lazy evaluation in languages like Haskell but with explicit control - Rust doesn't have pervasive laziness, just iterator-specific laziness with these methods providing early termination. This is more efficient than languages like Python where any/all are built-ins but still require predicate evaluation machinery, and clearer than manual loops with break statements scattered through imperative code. The design clearly communicates intent: any says "is there at least one," all says "do they all," and find says "give me the first" - each more readable than equivalent for-loop-with-flag implementations. The Option return from find elegantly handles the not-found case without exceptions or special values.

In practice, use find to locate specific elements (first error, first match, first occurrence), any to check existence (does this list contain forbidden values, are there any errors), and all to validate (are all inputs valid, do all elements meet criteria). These are essential for search, validation, and filtering. Common patterns include find for retrieving specific records, any for early-exit validation (if any invalid, reject all), and all for comprehensive checks (all must pass). The short-circuit behavior makes these efficient even on large collections - testing a million-element iterator with any stops at the first match. Be aware that find returns Option<&T> when using iter(), requiring dereferencing or cloning to get owned values. Also, these methods consume the iterator (taking self), though for searches this is usually fine since you're done after finding what you want or determining the answer."#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "custom_iterator",
            description: "Creating a custom iterator",
            code: r#"struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

let counter = Counter::new(5);
for num in counter {
    println!("Count: {}", num);
}"#,
            commentary: r#"Creating custom iterators by implementing the Iterator trait enables you to define domain-specific iteration patterns that integrate seamlessly with Rust's iterator ecosystem. The trait requires only two things: an associated type Item declaring what the iterator yields, and a next(&mut self) -> Option<Self::Item> method that produces values. Once you implement next(), you automatically inherit dozens of iterator methods (map, filter, fold, collect, etc.) for free through default implementations, making your custom iterator a first-class participant in iterator chains. This design makes iteration patterns data-structure-independent - whether iterating a tree, generating values, or reading from I/O, the same rich adapter library is available.

The technical implementation involves defining a struct to hold iteration state (current position, remaining elements, configuration, etc.) and implementing Iterator by providing next(). The &mut self parameter is crucial - it allows next() to mutate internal state, advancing through the sequence. Returning Option enables clean termination - Some(value) for the next element, None when exhausted. The associated type Item can be any type your iterator produces. For example, a range iterator holds start/end/current position and increments on each next(). The power comes from default trait implementations: once you have next(), the Iterator trait provides map, filter, and 75+ other methods automatically. These defaults build on next() - for example, the default count() implementation calls next() repeatedly until None.

Rust's trait-based iterator design achieves remarkable generality and composability. By requiring only next(), Rust makes custom iterators trivial to implement while providing a rich API for free. This is more elegant than C++'s iterator requirements (begin, end, operators, iterator categories) or Java's Iterator interface where you only get next/hasNext and must implement other operations manually. The associated type provides type safety - Iterator<Item=T> statically ensures all operations preserve types correctly. This design enables infinite iterators (std::iter::repeat), adapters that combine multiple sources (Chain, Zip), and domain-specific iteration (tree traversals, graph walks) with consistent ergonomics and zero-cost abstraction.

In practice, implement custom iterators for data structures (trees, graphs, custom collections), generation patterns (Fibonacci, random sequences, ranges with custom stepping), I/O sources (file line-by-line, network packets), and adapters that transform other iterators. The pattern is: define a struct with state, implement Iterator with next() logic, optionally implement size_hint() for optimization. For example, a tree iterator holds a stack of nodes for traversal, a CSV parser holds a reader and current position, a Fibonacci generator holds the last two numbers. Custom iterators enable elegant APIs: my_collection.iter() returns your iterator type, usable in for loops and with all iterator methods. Be aware that next() takes &mut self, so iterators are not Copy and can't be shared without cloning. Also consider implementing DoubleEndedIterator (next_back) for bidirectional iteration and ExactSizeIterator (len) when the size is known."#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "lazy_evaluation",
            description: "Iterators are lazy and zero-cost",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

// This doesn't execute until consumed!
let iterator = numbers
    .iter()
    .map(|x| {
        println!("Mapping {}", x);
        x * 2
    })
    .filter(|x| {
        println!("Filtering {}", x);
        *x > 5
    });

println!("Iterator created, now consuming:");
let result: Vec<i32> = iterator.collect();
println!("Result: {:?}", result);"#,
            commentary: r#"Lazy evaluation is the foundation of Rust's iterator efficiency and composability, meaning that iterator adapters like map and filter don't actually execute when called - they merely set up transformations to be applied later. The iterator chain does nothing until a consuming operation (collect, sum, for_each) pulls values through the pipeline, at which point each value flows through all transformation layers in a single pass. This laziness enables powerful compiler optimizations: the entire chain can be fused into a tight loop with no intermediate allocations or function call overhead. The "zero-cost abstraction" principle means that functional-style iterator code compiles to machine code as efficient as hand-written imperative loops, giving you expressiveness without performance penalty.

The technical implementation leverages Rust's type system and optimizer. Each adapter returns a wrapper type that implements Iterator by delegating to the wrapped iterator and applying its transformation. For example, Map<I, F> contains an I: Iterator and an F: Fn, with Map::next() calling I::next() and applying F to the result. Since these types are concrete (not trait objects), the compiler knows exactly what code to generate. With optimizations enabled, the compiler can see through the abstraction layers, inlining all function calls and eliminating intermediate iterator objects, often producing the same assembly as a manual for loop with inline transformations. The println! calls in the example demonstrate laziness - they only execute when collect() pulls values, not when the iterator chain is created.

Rust's commitment to zero-cost abstractions challenges the traditional systems programming assumption that high-level abstractions necessarily impose runtime overhead. While languages like Python or Java pay for abstraction through dynamic dispatch, boxing, and intermediate allocations, Rust's iterators are static constructs that completely disappear at runtime. This is similar to C++ template-heavy code but with better ergonomics and guaranteed safety. The design proves that you can have both expressiveness (declarative pipelines, composable operations) and performance (inline loops, no allocations) simultaneously. Lazy evaluation also enables infinite iterators - std::iter::repeat(x) creates an endless sequence, but since it's lazy, you only compute what you consume.

In practice, leverage lazy evaluation to build complex transformation pipelines without worrying about performance - the compiler will optimize them aggressively. Chain many adapters freely: iter().filter().map().filter().map() compiles to a single loop with all transformations inline. This code is often clearer and more maintainable than manual loops while being equally fast or faster (the optimizer can sometimes vectorize iterator chains but struggles with complex manual loops). Use infinite iterators for unbounded sequences, consuming only what you need: (0..).filter(is_prime).take(10) generates the first 10 primes. The key insight is that creating an iterator pipeline is cheap - it just sets up types and closures. The work happens during consumption. Be aware that lazy evaluation means side effects in adapters (like println!) won't execute until consumption, which can be surprising. For debugging, consume the iterator to see effects."#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "flat_map_flatten",
            description: "Flattening nested structures",
            code: r#"let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];

let flattened: Vec<i32> = nested.iter().flatten().copied().collect();
println!("Flattened: {:?}", flattened);

// flat_map: map then flatten
let words = vec!["hello", "world"];
let chars: Vec<char> = words.iter().flat_map(|s| s.chars()).collect();
println!("All chars: {:?}", chars);"#,
            commentary: r#"The flatten and flat_map adapters handle the common scenario of working with nested or hierarchical data structures, enabling elegant one-to-many transformations. flatten() takes an iterator of iterables (Iterator<Item = Iterator>) and produces a flat iterator over all inner elements, essentially concatenating all inner sequences into one continuous stream. flat_map(f) is the composition of map(f) followed by flatten() - it applies a function that returns an iterator for each element, then flattens all those iterators into a single sequence. This is invaluable when each input element expands to zero or more output elements, like splitting strings into characters, expanding nodes into children, or unpacking nested collections.

The technical implementation of flatten maintains state tracking the current inner iterator. When next() is called, flatten tries to get a value from the current inner iterator; if that's exhausted (returns None), it advances to the next inner iterator from the outer iterator and tries again, repeating until finding a value or exhausting the outer iterator. flat_map is implemented as a composition: FlatMap<I, F> holds a Map<I, F> and calls flatten on it, though the actual implementation is fused for efficiency. The critical insight is that flatten handles ragged inner iterators gracefully - some can be empty, others can have varying lengths, and it seamlessly chains them all. The type system ensures safety: Iterator<Item = Vec<T>> can be flattened because Vec<T> implements IntoIterator.

Rust's flatten and flat_map designs solve the nested iteration problem elegantly without requiring manual loop nesting or concatenation. In languages without these abstractions, you'd write nested for loops (for vec in vecs { for item in vec { ... } }), obscuring the conceptual flattening. flat_map is particularly powerful because it combines transformation and flattening in one operation, avoiding the intermediate iterator-of-iterators. This pattern comes from functional programming (Haskell's concatMap, Scala's flatMap) but Rust makes it zero-cost - the compiler can optimize flatten chains into simple nested iteration with no allocations. The design is more type-safe than languages where flatten might fail at runtime on non-iterable elements.

In practice, use flatten when you have nested collections and want a single flat sequence: nested vectors, Results containing Options, iterators of iterators. Common use cases include flattening tree structures into sequences, combining multiple data sources, and working with ragged arrays. Use flat_map when each element expands to multiple outputs: splitting strings into words or characters, expanding directories into files, generating permutations, or any one-to-many transformation. The pattern vec.iter().flat_map(|s| s.chars()) flattens a vector of strings into all characters. A powerful technique is using flat_map with Option or Result to filter and transform: some iterator.flat_map(|x| try_parse(x).ok()) transforms and removes failures in one step. Be aware that flatten requires the inner type to implement IntoIterator, and flat_map's closure must return something iterable."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
