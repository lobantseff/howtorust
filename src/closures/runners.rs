// Closure example runner functions organized in a separate file

pub fn run_example(name: &str) {
    match name {
        "basic_closure" => basic_closure(),
        "closure_type_inference" => closure_type_inference(),
        "capturing_environment" => capturing_environment(),
        "fn_traits" => fn_traits(),
        "move_keyword" => move_keyword(),
        "closures_as_parameters" => closures_as_parameters(),
        "returning_closures" => returning_closures(),
        "closure_caching" => closure_caching(),
        "closure_composition" => closure_composition(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_closure() {
    let add = |a, b| a + b;
    let result = add(5, 3);
    println!("5 + 3 = {}", result);

    let greet = |name| format!("Hello, {}!", name);
    println!("{}", greet("Alice"));
}

fn closure_type_inference() {
    let multiply = |x, y| x * y;
    println!("10 * 5 = {}", multiply(10, 5));

    let divide = |x: f64, y: f64| -> f64 { x / y };
    println!("10.0 / 3.0 = {}", divide(10.0, 3.0));
}

fn capturing_environment() {
    let x = 10;
    let y = 20;

    let add_to_x = |a| a + x;
    let add_both = |a| a + x + y;

    println!("5 + x = {}", add_to_x(5));
    println!("5 + x + y = {}", add_both(5));
}

fn fn_traits() {
    let x = vec![1, 2, 3];
    let print_vec = || println!("Vec: {:?}", x);
    print_vec();
    print_vec();

    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };
    increment();
    increment();

    let s = String::from("hello");
    let consume = || {
        let _owned = s;
        println!("Consumed string");
    };
    consume();
}

fn move_keyword() {
    let x = vec![1, 2, 3];

    let consume = move || {
        println!("Vec: {:?}", x);
    };

    consume();

    let y = String::from("Hello");
    let closure = move || {
        println!("{}", y);
    };
    closure();
}

fn closures_as_parameters() {
    fn apply_operation<F>(x: i32, f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }

    let double = |n| n * 2;
    let square = |n| n * n;

    println!("Double 5: {}", apply_operation(5, double));
    println!("Square 5: {}", apply_operation(5, square));
    println!("Add 10: {}", apply_operation(5, |n| n + 10));
}

fn returning_closures() {
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    let add_5 = make_adder(5);
    let add_10 = make_adder(10);

    println!("7 + 5 = {}", add_5(7));
    println!("7 + 10 = {}", add_10(7));
}

fn closure_caching() {
    struct Cacher<T>
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
    println!("Second call: {}", expensive.value(10));
}

fn closure_composition() {
    fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
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
    println!("(5 * 2) + 1 = {}", double_then_add(5));
}
