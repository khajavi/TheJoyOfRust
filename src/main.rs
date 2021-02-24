use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;

use crate::Number::Two;

fn main() {
    array_slice();
    enum_example();
    use_example();
    linked_list_example();
    constants_example();
    variable_binding_example();
    mutability();
    shadowing_example();
    delcare_first_initialize_later_example();
    freezing_examle();
    casting_example();
    literals_example();
    from_into_example();
    convert_to_string_example();
    parsing_string_example();
    expression_examples();
    loop_example();
    nesting_and_labels_example();
    return_from_loop();
    while_example();
    for_example();
    iter_mut_example();
    pointer_ref_destruction_example();
    if_let_example();
    while_let_example();
    methods_example();
    closure_example();
    capturing_example();
    closure_as_input_parameter();
    closure_input_functions();
    callback_manager_example();
    ownership_example1();
    ownership_example2();
    ownership_example3();
    ownership_example4();
    borrow_exmaple1();
    str_vs_String_example();
    generic_functions_example();
    generic_implementation_example();
    generic_bounds_example();
    generic_bound_with_where_clause_example();
    // callback_manager_2();
    box_example();
    event_listener_example();
    event_registry_example();
    lifetime_example_one();
    generic_lifetime_in_functions();
}


fn array_slice() {
    let array = [1, 2, 3, 4, 5];
    fn analyze_slice(slice: &[i32]) {
        println!("{}", slice[0])
    }
    analyze_slice(&array)
}

fn enum_example() {
    enum Day {
        First,
        Second,
    }

    fn inspect(day: Day) {
        match day {
            Day::First => println!("First"),
            Day::Second => println!("Second")
        }
    }

    inspect(Day::First)
}

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

#[derive(Debug)]
enum Number {
    One = 1,
    Two = 2,
    Three = 3,
}

fn use_example() {
    use crate::Status::Poor;
    let status = Poor;
    println!("status: {:?}", status);

    let number = Two;
    println!("number: {:?}", number);
    println!("number: {:?}", number as i32);
}

fn linked_list_example() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List { List::Nil }
        fn prepend(self, elem: i32) -> List {
            List::Cons(elem, Box::new(self))
        }
    }

    let empty = List::new();
    println!("{:?}", empty);
    println!("{:?}", empty.prepend(4).prepend(6));
}

fn constants_example() {
    const LANGUAGE: &str = "Rust";
    static PLATFORM: &str = "LINUX";

    println!("const: {}", LANGUAGE);
    println!("const: {}", PLATFORM)
}

fn variable_binding_example() {
    let integer_num = 4;
    let boolean_a = true;
    let unit = ();

    println!("{:?}, {:?}, {:?}", first = integer_num, second = boolean_a, third = unit)
}

fn mutability() {
    let mut counter = 0;
    counter = counter + 1;
    println!("{:?}", counter);
}

fn shadowing_example() {
    let shadow_binding = true;

    {
        let shadow_binding = 1;
        println!("{:?}", shadow_binding);
    }

    println!("{:?}", shadow_binding)
}

fn delcare_first_initialize_later_example() {
    let a_binding;

    a_binding = 3;
    println!("{:?}", a_binding);
}

fn freezing_examle() {
    let mut _mutable_integer = 5i32;

    {
        let _mutable_integer = _mutable_integer;
        // _mutable_integer = 6i32;
    }

    _mutable_integer = 6i32;
}

// Types

fn casting_example() {
    let decimal = 34.34352_351;
    let integer: i32 = decimal as i32;

    println!("{:?}", integer);
}

fn literals_example() {
    let x = 1u32;
    println!("std::mem::size_of_val(&x) = {:?}", std::mem::size_of_val(&x));
}

fn inference_example() {
    let mut vec = Vec::new();
    vec.push(3)
}

fn type_aliasing_example() {
    type Nanosecond = u64;
    type Inch = u64;
}

fn from_into_example() {
    let my_str = "Hello";
    let my_string = String::from(my_str);


    #[derive(Debug)]
    struct Number {
        value: i32
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("num = {:?}", num);
}

fn convert_to_string_example() {
    struct Circle {
        radius: i32
    }

    impl std::fmt::Display for Circle {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Circle with radius of {}", self.radius)
        }
    }

    let circle = Circle { radius: 40 };
    eprintln!("circle = {:?}", circle.to_string());
}

fn parsing_string_example() {
    let parsed = match "45".parse() {
        Ok(v) => v,
        Err(e) => 0
    };

    eprintln!("parsed = {:?}", parsed);
}

fn expression_examples() {
    let x = 5u32;

    let y = {
        3 + 3
    };

    let z = {
        5 + 3;
    };

    let u = {
        eprintln!("5 = {:?}", 5);
    };

    eprintln!("x = {:?}", x);
    eprintln!("y = {:?}", y);
    eprintln!("z = {:?}", z);
    eprintln!("u = {:?}", u);
}

fn loop_example() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter % 2 == 0 {
            eprintln!("counter = {:?}", counter);
            continue;
        } else if counter >= 40 {
            break;
        }
    }
}

fn nesting_and_labels_example() {
    let mut counter = 0;
    'outer: loop {
        counter += 1;
        eprintln!("outer with counter = {:?}", counter);

        if counter > 10 {
            break;
        } else {
            continue;
        }
        'inner: loop {
            println!("inner");
            break 'outer;
        }
    }
}

fn return_from_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    eprintln!("result = {:?}", result);
}

fn while_example() {
    let mut counter = 0;

    while counter < 5 {
        counter += 1;
        eprintln!("counter = {:?}", counter);
    }
}

fn for_example() {
    for n in 1..11 {
        eprintln!("n = {:?}", n);
    }

    for n in 1..=10 {
        eprintln!("n = {:?}", n);
    }

    let names = vec!["Foo", "Bar", "Baz"];

    for name in names.iter() {
        match name {
            &"Bar" => println!("Bar is contained"),
            &_ => eprintln!("name = {:?}", name)
        }
    }
}

fn iter_mut_example() {
    let mut mutable_names = vec!["Foo", "Bar", "Baz"];

    for name in mutable_names.iter_mut() {
        *name = match name {
            &mut "Bar" => "Baaaaar",
            &mut _ => name
        };
    }

    eprintln!("mutable_names = {:?}", mutable_names);
}

fn match_example() {}

fn tuple_destruction_example() {}

fn enum_destruction_example() {}

fn pointer_ref_destruction_example() {
    let reference = &4;

    match reference {
        &value => eprintln!("got value via destructing = {:?}", value)
    }

    //Dereferencing:
    match *reference {
        value => eprintln!("got a value from dereferencing = {:?}", value)
    }

    let value = 65;
    match value { ref r => eprintln!("got a reference to a value = {:?}", r) }

    let mut mut_value = 87;
    match mut_value {
        ref mut m => {
            *m += 10;
            eprintln!("we added 10, mut_value = {:?}", m);
        }
    }
    eprintln!("mut_value = {:?}", mut_value);
}

fn struct_destruction_example() {}

fn match_guard() {}

fn match_binding() {}

fn if_let_example() {
    let number = Some(34);

    if let Some(i) = number {
        eprintln!("i = {:?}", i);
    } else { println!("didn't matched"); }


    enum Event {
        Foo,
        Bar,
        Baz(i32),
    }

    let b = Event::Baz(54);

    if let Event::Baz(value) = b {
        eprintln!("value = {:?}", value);
    }
}

fn while_let_example() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("reached to bigger than 9");
            optional = None;
        } else {
            eprintln!("current i is = {:?}", i);
            optional = Some(i + 1)
        }
    }
}

fn function_example() {}

fn methods_example() {
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn print(&self) {
            println!("x: {:?}, y: {:?}", self.x, self.y);
        }

        fn new(x: i32, y: i32) -> Point {
            Point {
                x,
                y,
            }
        }
    }

    let point = Point::new(1, 2);
    point.print();
}

fn closure_example() {
    let inc_closure = |i: i32| -> i32{ i + 1 };
    let inc_closure_infered = |i| i + 1;
    let closure_with_no_arg = || 1;

    eprintln!("inc_closure(1) = {:?}", inc_closure(1));
    eprintln!("inc_closure_infered(1) = {:?}", inc_closure_infered(1));
    eprintln!("closure_with_no_arg() = {:?}", closure_with_no_arg());
}

fn capturing_example() {
    //TODO: read againt this section
    use std::mem;

    let color = String::from("green");
    let print = || eprintln!("color = {:?}", color);

    print();
}

fn closure_as_input_parameter() {
    // TODO: read again
    fn apply<F>(f: F) where
        F: FnOnce() {
        f()
    }

    fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 { f(3) }

    use std::mem;
    let greeting = "Hello";
    let mut farewell = "goodby".to_owned();

    let diary = || {
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzz");
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {:?}", apply_to_3(double));
}

fn closure_input_functions() {
    fn apply<F>(f: F) where
        F: Fn() {
        f()
    }

    let closure = || println!("Hello");

    apply(closure);
}


fn ownership_example1() {
    let x = String::from("Hello");
    let y = x;
    // eprintln!("x = {:?}", x);
    eprintln!("y = {:?}", y);
}

fn ownership_example2() {
    let x = String::from("Hello");
    fn work_with_string(s: String) {}
    work_with_string(x); // x moved here
    // eprintln!("x = {:?}", x); //borrow after moving
}

fn ownership_example3() {
    let x = String::from("Hello");
    fn work_with_string(s: String) {}
    eprintln!("x = {:?}", x); //borrow after moving
    work_with_string(x); // x moved here
    // eprintln!("x = {:?}", x); //we cant borrow moved value
}

fn ownership_example4() {
    let x = 4;
    fn work_with_i32(i: i32) {}
    work_with_i32(x); // x is not on the heap, so it copy the x not move it
    work_with_i32(x); // x is not on the heap, so it copy the x not move it.
}

fn borrow_exmaple1() {
    let x = String::from("Hello");
    let y = &x; // borrow
    let z = &x; // borrow
    fn work_with_string(s: &String) {}
    work_with_string(y);
    work_with_string(z);

    fn example_fn(take_ownership: String, read_only_borrow: &str, mutable_borrow: &mut str) {}
}

fn str_vs_String_example() {
    let string = String::from("Hello");
    eprintln!("string.len() = {:?}", string.len());
    eprintln!("string.capacity() = {:?}", string.capacity());

    let str = "Hello";
    eprintln!("str.len() = {:?}", str.len());
    // eprintln!("str.capacity() = {:?}", str.capacity()); // method not found

    fn foo_borrow_immutable_str(s: &str) {
        eprintln!("s inside foo = {:?}", s);
    }

    foo_borrow_immutable_str("Hello Foo");
}


fn generic_example() {
    struct A;
    struct Single(A);
    struct SingleGen<T>(T);

    let _s: Single = Single(A);
    let _char: SingleGen<char> = SingleGen('a');
    let _i32: SingleGen<i32> = SingleGen(352);
}

fn generic_functions_example() {
    struct A;
    struct S(A);
    struct SGen<T>(T);

    fn foo(_a: A) {}
    ;
    fn foo_gen_1(_s: SGen<A>) {}
    ;
    fn foo_gen_2(_s: SGen<char>) {}
    ;
    fn foo_gen<T>(_s: SGen<T>) {}
    ;

    foo(A);
    foo_gen_1(SGen(A));
    foo_gen_2(SGen('a'));
    foo_gen(SGen(355));
    foo_gen::<i32>(SGen(355));
}

fn generic_implementation_example() {
    struct S;
    struct GenericVal<T>(T);

    impl GenericVal<i32> {}
    impl GenericVal<S> {}
    impl<T> GenericVal<T> {}

    struct Val {
        val: i32,
    }

    impl Val {
        fn value(&self) -> &i32 {
            &self.val
        }
    }

    struct GenVal<T> {
        gen_val: T,
    }

    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 235 };
    let y = GenVal { gen_val: "hello" };
    eprintln!("x.value() = {:?}", x.value());
    eprintln!("y.value() = {:?}", y.value());
}

fn generic_trait_example() {
    struct Empty;
    struct Null;

    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }

    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {}
    }

    let empty = Empty;
    let null = Null;

    // empty.double_drop(empty);
}

fn generic_bounds_example() {
    fn printer<T: Display>(t: T) { println!("{}", t); }

    struct S<T: Display>(T);

    struct Rectangle { length: f64, height: f64 }

    trait HasArea { fn area(&self) -> f64; }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }

    fn area<T: HasArea>(t: &T) -> f64 { t.area() }

    let rectangle = Rectangle { length: 1.23, height: 53.5 };
    eprintln!("rectangle.area() = {:?}", rectangle.area());
}

fn generic_bound_with_where_clause_example() {
    fn printer<T>(t: T) where T: Display { println!("{}", t); }
    printer("Hello");

    fn print_option<T>(t: T) where Option<T>: Debug { println!("{:?}", Some(t)) }
    print_option("Hello")
}


fn callback_manager_example() {
    struct Cacher<T> where T: Fn(u32) -> u32 {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T> where T: Fn(u32) -> u32 {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation: calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
                Some(v) => v
            }
        }
    }

    let mut extensive_result = Cacher::new(|num| {
        println!("calculate slowly...");
        std::thread::sleep(Duration::from_millis(50));
        num
    });

    let res1 = extensive_result.value(4);
    let res2 = extensive_result.value(5);

    eprintln!("res1 = {:?}", res1);
    eprintln!("res2 = {:?}", res2);
}


// fn callback_manager_2() {
//     type Handler = i32;
//     type Closure = dyn FnOnce() -> i32;
//     
// 
//     struct CallbackManager {
//         current_key: Handler,
//         computations: HashMap<Handler, Box<Closure>>
//     }
// 
//     impl CallbackManager {
//         fn new() -> CallbackManager {
//             CallbackManager {
//                 current_key: 0,
//                 computations: HashMap::new(),
//             }
//         }
// 
//         fn add<F>(&mut self, callback: F) -> Handler where F: FnOnce() -> i32 {
//             self.current_key += 1;
//             self.computations.insert(self.current_key, Box::new(callback));
//             self.current_key
//         }
//     }
// 
//     let mut cbm = CallbackManager::new();
//     let cl = || 1;
//     eprintln!("cbm = {:?}", cbm.add(cl));
// }

fn box_example() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    let boxed_point = Box::new(Point { x: 1.23, y: 34.3 });
    eprintln!("boxed_point = {:?}", boxed_point);

    // pointer size:
    eprintln!("std::mem::size_of_val(&boxed_point) = {:?}", std::mem::size_of_val(&boxed_point));

    let unboxed_point = *boxed_point;
    eprintln!("std::mem::size_of_val(unboxed_poin) = {:?}", std::mem::size_of_val(&unboxed_point));
}

fn event_listener_example() {
    use std::any::Any;

    type EventListener = Box<dyn Fn(&dyn Any) -> i32>;

    #[derive(Default)]
    struct EventRegistry {
        listeners: HashMap<String, Vec<EventListener>>
    }

    impl EventRegistry {
        fn add_event_listener(&mut self, event: String, f: EventListener) {
            self.listeners.entry(event).or_insert_with(|| { Vec::new() }).push(f)
        }

        fn trigger(&self, event: String, data: &dyn Any) {
            let listeners = self.listeners.get(&event).unwrap();
            for listener in listeners.iter() {
                listener(data);
            }
        }
    }

    #[derive(Debug)]
    struct OnClick { mouse_x: f32, mouse_y: f32 }

    let mut events = EventRegistry::default();
    events.add_event_listener("click".to_owned(), Box::new(|event| {
        let event = event.downcast_ref::<OnClick>().unwrap();
        eprintln!("event = {:?}", event);
        assert_eq!(event.mouse_x, 1.);
        3
    }));

    events.add_event_listener("click".to_owned(), Box::new(|event| {
        let event = event.downcast_ref::<OnClick>().unwrap();
        eprintln!("eventss = {:?}", event);
        assert_eq!(event.mouse_x, 1.);
        4
    }));

    let event = OnClick { mouse_x: 1., mouse_y: 3. };
    events.trigger("click".to_owned(), &event);
}

fn event_registry_example() {
    type Handler = i32;
    type Callback = Box<dyn Fn() -> ()>;

    #[derive(Default)]
    struct CallbackManager {
        latest_handle: Handler,
        listeners: HashMap<Handler, Callback>,
    }

    impl CallbackManager {
        fn add(&mut self, closure: Callback) {
            self.latest_handle += 1;
            self.listeners.insert(self.latest_handle, closure);
        }

        fn run_all(&self) {
            for (i, c) in self.listeners.iter() {
                c();
            }
        }
    }

    let mut cbm = CallbackManager::default();
    cbm.add(Box::new(|| println!("Hello")));
    cbm.add(Box::new(|| println!("Hello, World!")));
    eprintln!("cbm.listeners = {:?}", cbm.listeners.keys());
    cbm.run_all();
}

fn lifetime_example_one() {
    let r;
    {
        let x = 5;
        // uncomment below to error
        // r = &x;
    }
    r = 2;
    eprintln!("r = {:?}", r);
}

fn generic_lifetime_in_functions() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
}


