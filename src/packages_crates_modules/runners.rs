// Packages, crates, and modules example runner functions organized in a separate file

pub fn run_example(name: &str) {
    match name {
        "basic_module" => basic_module(),
        "nested_modules" => nested_modules(),
        "use_keyword" => use_keyword(),
        "use_as" => use_as(),
        "pub_use" => pub_use(),
        "glob_imports" => glob_imports(),
        "module_visibility" => module_visibility(),
        "super_keyword" => super_keyword(),
        "struct_privacy" => struct_privacy(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_module() {
    mod sound {
        pub fn guitar() {
            println!("🎸 Guitar sound!");
        }
    }

    sound::guitar();
}

fn nested_modules() {
    use crate::packages_crates_modules::garden::vegetables::Carrot;

    let carrot = Carrot::new(15);
    println!("Carrot length: {} cm", carrot.length_cm);
}

fn use_keyword() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 50);
    println!("Scores: {:?}", scores);
}

fn use_as() {
    use std::collections::HashMap as Map;

    let mut data = Map::new();
    data.insert("key", "value");
    println!("{:?}", data);
}

fn pub_use() {
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("Added to waitlist");
            }
        }
    }

    pub use front_of_house::hosting;

    hosting::add_to_waitlist();
}

fn glob_imports() {
    use std::collections::*;

    let mut map = HashMap::new();
    map.insert(1, "one");

    let mut set = HashSet::new();
    set.insert(42);

    println!("HashMap: {:?}, HashSet: {:?}", map, set);
}

fn module_visibility() {
    mod my_mod {
        pub fn public_fn() {
            println!("Public function");
            private_fn();
        }

        fn private_fn() {
            println!("Private function called from within");
        }
    }

    my_mod::public_fn();
}

fn super_keyword() {
    mod parent {
        pub fn parent_function() {
            println!("Called from parent");
        }

        pub mod child {
            pub fn child_function() {
                println!("Called from child");
                super::parent_function();
            }
        }
    }

    parent::child::child_function();
}

fn struct_privacy() {
    mod shapes {
        pub struct Rectangle {
            pub width: u32,
            height: u32,
        }

        impl Rectangle {
            pub fn new(width: u32, height: u32) -> Self {
                Rectangle { width, height }
            }

            pub fn area(&self) -> u32 {
                self.width * self.height
            }
        }
    }

    let rect = shapes::Rectangle::new(30, 50);
    println!("Width: {}, Area: {}", rect.width, rect.area());
}
