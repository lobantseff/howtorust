// Error handling example runner functions organized in a separate file

pub fn run_example(name: &str) {
    match name {
        "basic_result" => basic_result(),
        "basic_option" => basic_option(),
        "unwrap_and_expect" => unwrap_and_expect(),
        "question_mark_operator" => question_mark_operator(),
        "option_combinators" => option_combinators(),
        "result_combinators" => result_combinators(),
        "custom_error_types" => custom_error_types(),
        "error_conversion" => error_conversion(),
        "multiple_error_types" => multiple_error_types(),
        "early_return_pattern" => early_return_pattern(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_result() {
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn basic_option() {
    fn find_user(id: u32) -> Option<String> {
        if id == 1 {
            Some(String::from("Alice"))
        } else {
            None
        }
    }

    match find_user(1) {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }

    match find_user(999) {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }
}

fn unwrap_and_expect() {
    let some_value: Option<i32> = Some(42);
    let value = some_value.unwrap();
    println!("Value: {}", value);

    let result: Result<i32, &str> = Ok(100);
    let num = result.expect("Failed to get number");
    println!("Number: {}", num);
}

fn question_mark_operator() {
    fn read_number(text: &str) -> Result<i32, std::num::ParseIntError> {
        let num: i32 = text.parse()?;
        Ok(num * 2)
    }

    match read_number("42") {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match read_number("not a number") {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn option_combinators() {
    let some_string = Some("42");

    let doubled = some_string
        .map(|s| s.parse::<i32>().ok())
        .flatten()
        .map(|n| n * 2);

    println!("Doubled: {:?}", doubled);

    let result = Some(5)
        .and_then(|n| if n > 0 { Some(n * 2) } else { None })
        .or(Some(0));

    println!("Result: {:?}", result);
}

fn result_combinators() {
    let result: Result<i32, &str> = Ok(10);

    let doubled = result.map(|n| n * 2);
    println!("Doubled: {:?}", doubled);

    let error_result: Result<i32, &str> = Err("oops");
    let with_default: Result<i32, &str> = error_result.or(Ok(0));
    println!("With default: {:?}", with_default);

    let error_result2: Result<i32, &str> = Err("oops");
    let mapped_err = error_result2.map_err(|e| format!("Error: {}", e));
    println!("Mapped error: {:?}", mapped_err);
}

fn custom_error_types() {
    use std::fmt;

    #[derive(Debug)]
    #[allow(dead_code)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
    }

    impl fmt::Display for MathError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
                MathError::NegativeSquareRoot => {
                    write!(f, "Cannot take square root of negative")
                }
            }
        }
    }

    fn divide_checked(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }

    match divide_checked(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn error_conversion() {
    use std::num::ParseIntError;

    #[derive(Debug)]
    #[allow(dead_code)]
    enum AppError {
        ParseError(ParseIntError),
        Custom(String),
    }

    impl From<ParseIntError> for AppError {
        fn from(err: ParseIntError) -> Self {
            AppError::ParseError(err)
        }
    }

    fn process_number(s: &str) -> Result<i32, AppError> {
        let num: i32 = s.parse()?;
        if num < 0 {
            Err(AppError::Custom(
                "Negative numbers not allowed".into(),
            ))
        } else {
            Ok(num)
        }
    }

    match process_number("42") {
        Ok(n) => println!("Number: {}", n),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn multiple_error_types() {
    use std::error::Error;

    fn parse_demo(text: &str) -> Result<i32, Box<dyn Error>> {
        let num: i32 = text.trim().parse()?;
        Ok(num)
    }

    match parse_demo("123") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn early_return_pattern() {
    fn validate_and_process(value: i32) -> Result<i32, String> {
        if value < 0 {
            return Err("Value must be non-negative".into());
        }

        if value > 100 {
            return Err("Value must be <= 100".into());
        }

        if value % 2 != 0 {
            return Err("Value must be even".into());
        }

        Ok(value * 2)
    }

    for val in [10, -5, 150, 7, 20] {
        match validate_and_process(val) {
            Ok(result) => println!("{} -> {}", val, result),
            Err(e) => println!("{} -> Error: {}", val, e),
        }
    }
}
