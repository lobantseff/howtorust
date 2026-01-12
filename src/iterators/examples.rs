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
            commentary: r#"📚 INTRODUCTION
Iterators provide a unified abstraction for processing
sequences of values one at a time. The iter() method
creates an iterator that borrows elements immutably, and
for loops are syntactic sugar that desugar to
iterator.next() calls until None is returned.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to other languages:
- Python: Iteration involves runtime protocol dispatch and
boxed values
- Java: Iterators allocate objects and require
hasNext()/next() ceremony
- C++: STL iterators need separate begin/end pairs and
pointer arithmetic

Problems prevented:
- Manual index management and off-by-one errors
- Inconsistent iteration APIs across different collection
types
- Runtime overhead from dynamic dispatch and allocations
- Unsafe pointer arithmetic when traversing sequences

🔍 IMPORTANT DETAILS & INTRICACIES
The Iterator trait requires implementing next(&mut self) ->
Option<Self::Item>. The &mut self parameter means iterators
track internal state and mutate on each call. Collections
provide three iterator methods: iter() for borrowing,
iter_mut() for mutable borrowing, and into_iter() for
consuming ownership. Calling iter() multiple times creates
independent iterators with separate state. Iterators are
lazy - they do nothing until consumed by a loop or terminal
operation. The design unifies external iteration (you pull
values), internal iteration (for_each with closures), and
lazy iteration through adapters.

💼 WHERE IT'S MOST USED
- Sequential processing of collections without consuming
them
- Numeric iteration with ranges (0..10) replacing C-style
for loops
- Processing file lines, network streams, or other I/O
sources
- Building transformation pipelines with adapters (covered
in later examples)
- Any scenario requiring repeated access to elements

✅ TAKEAWAY
Rust's iterators are zero-cost abstractions that compile to
code as efficient as hand-written indexed loops, often
better due to optimizer opportunities. The Iterator trait
unifies iteration across all types - vectors, ranges,
files, custom data structures - providing consistent, safe,
and performant sequential access without runtime overhead,
manual index tracking, or pointer arithmetic."#,
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
            commentary: r#"📚 INTRODUCTION
Iterator adapters like map and filter are building blocks
for functional-style data transformation. map transforms
each element by applying a function, filter selectively
passes elements matching a predicate, and collect() gathers
results into a collection. These adapters are lazy - they
return new iterator types that perform transformations only
when consumed.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to other languages:
- Python 2: map/filter created intermediate lists, wasting
memory
- Python 3: map/filter are lazy but have runtime overhead
and boxing
- Java Streams: Require boxing, virtual calls, and stream
creation overhead
- C++: Algorithms require begin/end pairs and are harder to
chain

Problems prevented:
- Intermediate collection allocations during
transformations
- Imperative loops with nested logic that's hard to read
and maintain
- Manual index tracking when transforming data
- Accidental mutation of source collections during
processing

🔍 IMPORTANT DETAILS & INTRICACIES
Each adapter returns a new iterator type: map returns
Map<I, F>, filter returns Filter<I, P>. These implement
Iterator by delegating to the wrapped iterator's next() and
applying transformations. The compiler optimizes entire
chains into single loops with no intermediate allocations.
The collect() method uses FromIterator trait to build
various collection types, determined by type inference or
turbofish syntax ::<Vec<_>>. The copied() or cloned()
adapters convert &T to T when needed after operations.
Remember: iter() yields &T while into_iter() yields T,
affecting closure parameters and dereference needs.

💼 WHERE IT'S MOST USED
- Transforming data: converting types, extracting fields,
applying calculations
- Filtering: removing nulls, finding matches, validating
conditions
- Building pipelines:
iter().filter(pred).map(transform).collect()
- Processing API responses, database results, or file
contents
- ETL operations where data flows through transformation
stages

✅ TAKEAWAY
Rust's iterator adapters bring functional programming's
declarative expressiveness to systems programming with zero
runtime cost. Unlike Python's lazy objects or Java's Stream
API, Rust's adapters compile to efficient inline loops that
the optimizer can vectorize with SIMD instructions. The
lazy evaluation enables composable, readable transformation
pipelines that directly express what you want rather than
how to get it, while maintaining performance equivalent to
hand-written imperative loops."#,
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
            commentary: r#"📚 INTRODUCTION
Consuming adapters are terminal operations that take
ownership of iterators and pull all values through to
produce a final result. Methods like sum(), product(),
count(), and collect() immediately execute the entire
iterator chain, aggregating values into a single output.
These take self (by value), making the iterator unusable
afterward.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to other languages:
- Python: sum/len work on lists but require full iteration
without signaling consumption
- JavaScript: reduce is the general pattern, but no
specialized sum/product operations
- Java: Streams have terminal operations but with boxing
overhead for primitives
- C++: std::accumulate works but doesn't prevent iterator
reuse

Problems prevented:
- Attempting to reuse exhausted iterators (compile-time
prevention via move)
- Runtime checks for iterator exhaustion
- Unclear resource ownership and lifetime management
- Inefficient multiple passes when one suffices

🔍 IMPORTANT DETAILS & INTRICACIES
sum() and product() require the Item type to implement Sum
or Product traits respectively, ensuring type safety. These
methods call next() repeatedly, accumulating results. Type
inference determines output type from context; use
turbofish ::<i32>() when needed. Chaining lazy adapters
before consuming enables one-pass processing:
numbers.iter().map(|x| x * 2).sum() computes efficiently
with no intermediate allocations. The compiler fuses
transformations into tight loops. Taking self instead of
&self makes resource usage explicit in the type system,
preventing reuse bugs at compile time.

💼 WHERE IT'S MOST USED
- Aggregating numerical data: totals with sum(), products
for factorial calculations
- Counting elements: count() for collection sizes, filtered
counts
- Finding extremes: max()/min() for largest/smallest values
- Collecting results: gather into Vec, HashMap, HashSet, or
custom collections
- Final stage of transformation pipelines after map/filter
chains

✅ TAKEAWAY
Rust's consuming adapters make iterator exhaustion explicit
through ownership semantics - taking self prevents
accidental reuse and enables aggressive compiler
optimizations. The pattern of chaining lazy adapters
followed by a terminal consumer (filter().map().sum())
processes elements in one efficient pass with no
intermediate allocations, while the type system ensures you
can't mistakenly reuse exhausted iterators, eliminating an
entire class of bugs that plague languages without move
semantics."#,
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
            commentary: r#"📚 INTRODUCTION
Iterator adapter chaining enables complex data
transformations expressed as readable, composable
pipelines. Each adapter returns a new iterator type,
allowing fluent method chaining:
iter().filter().map().filter(). The entire chain remains
lazy until a consuming operation triggers evaluation,
processing values through all layers in a single pass.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to other languages:
- Java (pre-Stream): Method chaining created intermediate
collections, wasting memory
- Ruby: Chained enumerable methods allocate intermediates
for each step
- Scala: Provides similar chaining but with JVM overhead
and garbage collection
- C++20 ranges: Similar concept but less mature API and
more complex syntax

Problems prevented:
- Intermediate collection allocations between
transformation steps
- Deeply nested loops with complex indentation and
scattered logic
- Manual index management across multiple transformation
stages
- Performance penalty from high-level abstractions

🔍 IMPORTANT DETAILS & INTRICACIES
The compiler generates nested iterator wrappers:
iter().filter(f).map(g) becomes approximately
Map<Filter<Iter, F>, G>. Each wrapper implements Iterator
by delegating to its inner iterator and applying
transformations. The compiler sees through these layers,
inlining everything into a single efficient loop with no
intermediate collections. Values flow through one at a time
with transformations applied inline. Type safety is
preserved: if map produces Iterator<Item=T>, the next
filter receives &T. Use inspect(|x| println!("{:?}", x))
for debugging to observe values at any pipeline point.

💼 WHERE IT'S MOST USED
- ETL pipelines: extract, filter invalid data, transform,
collect results
- Data processing: filter().map().filter() patterns for
multi-stage refinement
- API response handling: parse, validate, transform,
aggregate data
- File processing: read lines, filter comments, parse,
collect
- Any scenario requiring multi-step sequential
transformations

✅ TAKEAWAY
Rust's iterator chaining brings functional programming's
elegance to systems programming with zero runtime cost.
Complex transformations become linear, self-documenting
pipelines that read like English descriptions while
compiling to efficient single-pass loops. The lazy
evaluation ensures no intermediate allocations, and the
compiler's aggressive optimization often produces code
superior to hand-written loops, proving that declarative
style and performance aren't mutually exclusive."#,
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
            commentary: r#"📚 INTRODUCTION
fold and reduce are fundamental accumulation operations
that combine iterator elements by repeatedly applying a
binary function. fold(initial, closure) takes an explicit
starting value and threads state through iteration.
reduce(closure) uses the first element as initial
accumulator and returns Option<T> for empty iterators. fold
is the "universal aggregator" that can implement any
consuming operation.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to other languages:
- JavaScript: reduce is similar but with looser type safety
and no ownership semantics
- Haskell: foldl/foldr are fundamental but use implicit
patterns vs Rust's explicit syntax
- Python: functools.reduce exists but is less ergonomic and
has runtime overhead
- C++: std::accumulate provides similar functionality but
less type-safe

Problems prevented:
- Manual accumulation with mutable variables and explicit
loop management
- Boilerplate for common aggregation patterns (sum, max,
concatenation)
- Type errors when combining incompatible types
- Off-by-one errors in manual accumulation loops

🔍 IMPORTANT DETAILS & INTRICACIES
fold maintains accumulator state, calling the closure for
each element with (accumulator, element) and updating with
the result. The accumulator type can differ from Item type,
enabling transformations like numbers.fold(Vec::new(), |mut
acc, x| { acc.push(x * 2); acc }). reduce uses the first
element as initial accumulator and returns Option<T> for
empty iterators. Choose initial values carefully: 0 for
addition, 1 for multiplication, empty collection for
building structures. Both consume the iterator completely.
The closure's accumulator parameter can move or borrow
depending on types and ownership needs.

💼 WHERE IT'S MOST USED
- Custom aggregations: running totals with additional logic
beyond simple sum
- Building data structures: accumulating into HashMap,
String with separators
- Stateful transformations: tracking counts, positions, or
conditions across elements
- Pairwise combinations: finding max/min, concatenating,
merging
- Implementing domain-specific reduction operations

✅ TAKEAWAY
fold and reduce encapsulate the accumulation pattern from
functional programming, making state-threading operations
composable and optimization-friendly. fold's power comes
from its generality - the accumulator can have any type,
enabling transformations from sequences to arbitrary
structures. This is the foundation for sum, product,
collect, and other consuming operations, proving that a
simple abstraction (repeatedly apply a function to
accumulator and element) can express virtually any
aggregation with type safety and zero-cost performance."#,
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
            commentary: r#"📚 INTRODUCTION
enumerate and zip coordinate iteration across multiple data
sources or add positional information. enumerate() yields
(usize, T) tuples pairing zero-based indices with elements.
zip() combines two iterators into (A, B) tuples of
corresponding elements, stopping when either iterator
exhausts. These eliminate manual index tracking and unsafe
parallel iteration.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to other languages:
- Python: enumerate/zip create tuple objects with runtime
overhead
- C++: Requires explicit index loops or complex iterator
pairing
- JavaScript: No built-in enumerate, zip requires lodash or
manual implementation
- Java: No built-in zip, enumerate requires manual counter
management

Problems prevented:
- Manual index variables (let mut i = 0) leading to
off-by-one errors or forgotten increments
- Unsafe parallel array indexing (arr1[i], arr2[i]) when
lengths differ
- Index-out-of-bounds errors when iterating multiple
sequences
- Verbose boilerplate for common coordination patterns

🔍 IMPORTANT DETAILS & INTRICACIES
enumerate maintains an internal counter starting at zero,
incrementing with each next() call and pairing with
elements. zip holds both iterators, calls next() on each,
and returns Some((a, b)) only if both succeed or None if
either exhausts. Early termination on the shorter sequence
is deliberate, preventing undefined access. Tuples
destructure cleanly: for (i, val) in iter.enumerate() or
for (a, b) in iter1.zip(iter2). These compile to efficient
code with no runtime overhead - enumerate is just a counter
increment, zip is dual next() calls.

💼 WHERE IT'S MOST USED
- Position-aware processing: indexing other structures,
progress reporting, conditional logic
- Parallel array processing: coordinates (x/y), combining
keys with values
- Element-wise operations: comparing sequences, arithmetic
on paired vectors
- CSV/table processing: pairing headers with data rows
- Inserting separators: commas between elements except the
last

✅ TAKEAWAY
Rust's enumerate and zip provide zero-cost abstractions for
common iteration coordination patterns, compiling to simple
counter increments and dual next() calls with no runtime
overhead. They eliminate error-prone manual index
management and unsafe parallel array access while providing
compile-time type safety for tuple elements. The zip
design's early termination on the shorter sequence prevents
out-of-bounds errors, making these abstractions both safer
and more elegant than explicit indexing approaches in other
languages."#,
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
            commentary: r#"📚 INTRODUCTION
find, any, and all provide efficient search and predicate
testing with built-in short-circuit evaluation.
find(predicate) returns Option<T> with the first matching
element. any(predicate) returns bool if at least one
element matches. all(predicate) returns bool if every
element matches. Short-circuiting stops iteration as soon
as the answer is determined.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to other languages:
- Python: any/all are built-ins but with predicate
evaluation overhead and no find
- JavaScript: find exists but no any/all (replaced by
some/every), less efficient
- Java: anyMatch/allMatch on Streams but with boxing
overhead for primitives
- C++: std::find exists but no short-circuit any/all,
requires manual algorithms

Problems prevented:
- Manual loops with break statements and flag variables
scattered through code
- Unnecessary iteration over remaining elements after
answer is determined
- Verbose for-loop-with-flag implementations obscuring
intent
- Exception handling or special values for not-found cases
(Option is clearer)

🔍 IMPORTANT DETAILS & INTRICACIES
Each method calls next() repeatedly, testing predicates on
elements. find returns Some(element) immediately on first
match or None if exhausted. any returns true on first true
predicate or false if none match. all returns false on
first false predicate or true if all match.
Short-circuiting is automatic - no manual break needed.
Predicate closures receive &T when using iter(), allowing
inspection without consuming the collection. Methods
consume the iterator itself (taking self). find returns
Option<&T> with iter(), requiring dereference or clone for
owned values.

💼 WHERE IT'S MOST USED
- Search operations: finding first error, first match,
specific records by criteria
- Existence checks: does list contain forbidden values, are
there any errors
- Validation: are all inputs valid, do all elements meet
requirements
- Early-exit logic: if any invalid, reject entire batch
- Comprehensive checks: all must pass before proceeding

✅ TAKEAWAY
Rust's search methods combine functional programming
expressiveness with optimal performance through automatic
short-circuit evaluation. They clearly communicate intent
(any="is there at least one", all="do they all", find="give
me the first") while being more efficient than manual
loops. The Option return from find elegantly handles
not-found cases without exceptions, and short-circuiting
makes these methods efficient even on large collections,
stopping at the first match rather than wastefully
processing remaining elements."#,
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
            commentary: r#"📚 INTRODUCTION
Custom iterators integrate domain-specific iteration
patterns with Rust's iterator ecosystem. The Iterator trait
requires only two things: an associated type Item and a
next(&mut self) -> Option<Self::Item> method. Once
implemented, you inherit 75+ iterator methods (map, filter,
fold, collect) for free through default implementations.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to other languages:
- C++: Iterators require begin, end, operators, and
iterator categories - verbose and error-prone
- Java: Iterator interface provides only next/hasNext, must
manually implement other operations
- Python: __iter__ and __next__ work but lack static type
safety and have runtime overhead
- JavaScript: Symbol.iterator provides similar
functionality but with dynamic typing

Problems prevented:
- Implementing dozens of methods manually for each iterator
type
- Inconsistent iteration APIs across different data
structures
- Boilerplate for common operations on custom sequences
- Runtime overhead from dynamic dispatch or boxing

🔍 IMPORTANT DETAILS & INTRICACIES
Define a struct holding iteration state (position,
configuration, remaining elements). Implement Iterator with
next() logic - &mut self allows mutating state, Option
enables clean termination. Associated type Item can be
anything your iterator produces. Default implementations
build on next(): count() calls it until None, map wraps it,
etc. Consider implementing size_hint() for optimization,
DoubleEndedIterator (next_back) for bidirectional
iteration, and ExactSizeIterator (len) when size is known.
Iterators aren't Copy (next takes &mut self) and can't be
shared without cloning.

💼 WHERE IT'S MOST USED
- Custom data structures: tree/graph traversals, custom
collection iteration
- Generation patterns: Fibonacci sequences, random numbers,
custom ranges
- I/O sources: file lines, network packets, streaming data
- Transformation adapters: combining multiple iterators,
custom filters
- Domain-specific sequences: date ranges, game state
progression

✅ TAKEAWAY
Rust's trait-based iterator design achieves remarkable
generality by requiring only next() implementation while
providing 75+ methods automatically through defaults. This
is far more elegant than C++'s complex iterator
requirements or Java's minimal interface requiring manual
implementations. The associated type ensures static type
safety, and the design enables infinite iterators,
multi-source adapters, and domain-specific iteration with
consistent ergonomics and zero-cost abstraction - whether
iterating a tree, generating values, or reading I/O, the
same rich API is available."#,
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
            commentary: r#"📚 INTRODUCTION
Lazy evaluation is the foundation of iterator efficiency -
adapters like map and filter don't execute when called,
they just set up transformations. The iterator chain does
nothing until a consuming operation (collect, sum,
for_each) pulls values through. This enables powerful
compiler optimizations: chains fuse into tight loops with
no intermediate allocations or function call overhead.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to other languages:
- Python 2: map/filter executed immediately, creating
intermediate lists wastefully
- Java: Stream operations are lazy but have boxing overhead
and virtual calls
- JavaScript: Most array methods (.map, .filter) create
intermediate arrays
- Haskell: Lazy by default but with garbage collection
overhead

Problems prevented:
- Intermediate collection allocations for each
transformation step
- Wasted computation on elements that get filtered out
later
- Performance penalty from high-level functional
abstractions
- Memory overhead from storing intermediate results

🔍 IMPORTANT DETAILS & INTRICACIES
Each adapter returns a wrapper type implementing Iterator
by delegating and transforming. Map<I, F> contains Iterator
I and function F, with Map::next() calling I::next() and
applying F. Types are concrete (not trait objects), so the
compiler knows exact code generation. With optimizations,
the compiler inlines all calls and eliminates iterator
objects, producing assembly identical to manual loops. Side
effects in adapters (println!) only execute during
consumption, not chain creation. Infinite iterators like
repeat(x) are possible - you only compute what you consume.

💼 WHERE IT'S MOST USED
- Building complex transformation pipelines: multiple
map/filter stages
- Processing large datasets where intermediate allocations
would be costly
- Infinite sequences: (0..).filter(is_prime).take(10) for
first 10 primes
- Stream processing: consume elements on-demand from I/O
sources
- Performance-critical code where zero-cost abstraction is
essential

✅ TAKEAWAY
Rust's lazy iterators prove you can have both
expressiveness and performance. Functional-style pipelines
compile to machine code as efficient as hand-written loops
through static types and aggressive optimization. The
compiler fuses entire chains into single loops with no
runtime overhead, challenging the assumption that
high-level abstractions impose performance penalties. This
is zero-cost abstraction in action: declarative code that
completely disappears at runtime, often producing better
assembly than manual loops because the optimizer can
vectorize uniform iterator chains."#,
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
            commentary: r#"📚 INTRODUCTION
flatten and flat_map handle nested or hierarchical data
structures, enabling one-to-many transformations. flatten()
takes an iterator of iterables and produces a flat stream
over all inner elements. flat_map(f) applies a function
returning an iterator for each element, then flattens all
results into a single sequence.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Compared to other languages:
- JavaScript: flatMap exists but creates intermediate
arrays with memory overhead
- Python: itertools.chain.from_iterable provides similar
functionality but less ergonomic
- Haskell: concatMap is fundamental but with garbage
collection overhead
- Java: flatMap on Streams has boxing overhead for
primitives

Problems prevented:
- Nested for loops obscuring conceptual flattening
operations
- Manual concatenation of sequences with boilerplate
- Intermediate iterator-of-iterators allocations
- Runtime failures on non-iterable elements (Rust's type
system prevents this)

🔍 IMPORTANT DETAILS & INTRICACIES
flatten maintains state tracking the current inner
iterator. On next(), it tries the current inner iterator;
if exhausted, advances to the next inner iterator and
repeats until finding a value or exhausting the outer
iterator. flat_map fuses map and flatten for efficiency.
flatten handles ragged inner iterators gracefully - varying
lengths, empty sequences. Type system ensures safety:
Iterator<Item = Vec<T>> can flatten because Vec<T>
implements IntoIterator. Powerful pattern:
iterator.flat_map(|x| try_parse(x).ok()) filters and
transforms, removing failures in one step.

💼 WHERE IT'S MOST USED
- Nested collections: flattening Vec<Vec<T>> into flat
sequences
- String processing: vec.iter().flat_map(|s| s.chars()) for
all characters
- Tree/graph traversal: expanding nodes into children
sequences
- Parsing: filter and transform with flat_map(|x|
parse(x).ok())
- Directory traversal: expanding directories into files

✅ TAKEAWAY
Rust's flatten and flat_map elegantly solve nested
iteration without manual loop nesting, combining
transformation and flattening in zero-cost operations. The
compiler optimizes chains into simple nested iteration with
no intermediate allocations, bringing functional
programming patterns (Haskell's concatMap, Scala's flatMap)
to systems programming. The type system provides
compile-time safety against non-iterable elements, and the
graceful handling of ragged sequences makes these adapters
essential for one-to-many transformations, hierarchical
data processing, and combined filter-transform operations."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
