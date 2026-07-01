use std::collections::HashMap;

// Entry point
fn main() {
    println!("Hello, world!");

    // ------Primitive Data types----- //

    // Some theory
    // A memory address is represented by address bits
    // memory locations = 2^no. of address bits
    // a i8 simply mean it can store a signed integer of 1 byte

    // A 32-bit CPU means its general-purpose registers are 32 bits wide.
    // It can efficiently process 32-bit values in a single instruction.
    // (Some instructions can still operate on smaller or larger values.)

    // If a CPU has a 32-bit address space, it can generate 2^32 unique addresses.
    // Since each address refers to 1 byte, it can directly address up to 4 GB of memory.

    // ---- int ----
    // i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // Architecture-dependent	isize	usize

    let e: i8 = 127;
    println!("Yay integer: {}", e);

    // ------ floats ----
    // f32, f64
    let pi: f64 = 3.14;
    println!("Yay floats: {}", pi);

    // ----- bool ----
    let is_awesome: bool = true;
    println!("Is Awesome? {}", is_awesome);

    // ---- char (unicode) ----
    let letter: char = 'a';
    println!("Letter {}", letter);

    // -------- Compound Data Types ---------- //

    // ---- Arrays ----
    // Homogenous values
    let arr: [i32; 3] = [1, 2, 3];
    println!("First value {}", arr[0]);
    println!("Array {:?}", arr);

    // ---- Tuple -----
    // Heterogenous grouped data
    let tup: (&str, u32) = ("Hello", 23);
    println!("Tupple value {}", tup.1); // No dynamic indexing possible since there can be padding and no proper way to determine address at runtime

    // ----- slices --> A view of a contagious data type ------
    let arr: [i32; 3] = [1, 2, 3];
    let _num_slice: &[i32] = &[1, 2, 3][0..1];
    let num_slice: &[i32] = &arr[0..1]; // Just a slice of array
    println!("Slice {:?}", num_slice);

    // We can create more slices like this
    let my_array_of_tup: [(String, i32, bool); 2] = [
        ("Hi".to_string(), 32, true),
        ("Hello".to_string(), 34, false),
    ];

    let my_slice: &[(String, i32, bool)] = &my_array_of_tup[0..1];
    println!("New Slice {:?}", my_slice);

    // ----- Strings --> [Growable, Mutable, Stored in Heap, utf-8 encoded] -----

    let mut my_string: String = String::from("Hello!!");
    my_string.push_str(" World!!");

    println!("{}", my_string);

    // Similarly we can create a string slice too [immutable, in stack, utf-8 encoded]
    // &str references the str bytes in heap
    // &String --> It references the String struct and can be used as a string (immutable again as references are always immutables)
    let my_str_slice: &str = &my_string[0..5];
    println!("String slice {}", my_str_slice);

    hello_world();

    let length: u32 = 23;
    let width: u32 = 4;

    println!("Area of rectangle is {} sqcm", area_of_rect(length, width));

    //
    // Rust is memory safe, no need to manually allocate and free memory
    // And no use of garbage collectors as it freeze the program during cleaning

    // --------- Ownership ----------- //
    // Every value has a single owner in Rust

    // Rules of Ownership in Rust
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // Not that expensive so follows Copy trait
    let a: i32 = 23;
    let b: i32 = a;
    println!("{}", a);
    println!("{}", b);

    // Expensive hence transfers ownership
    // let my_var: String = String::from("Hello");
    // let my_var_owned: String = my_var;
    // println!("This is owned newly {}", my_var_owned);
    // println!("This was owned previously {}", my_var);

    // ------------- Borrowing ------------- //
    let i = 5;
    let _borrowed_i = &i; // Borrowed references are immutable by default
    // *borrowed_i += 1; Not possible since immutable

    let mut mutable_i = 5;
    let mutable_borrowed_i = &mut mutable_i;

    *mutable_borrowed_i += 9;

    // println!("Mutable i {}", mutable_i); mutable borrow exists, no immutable borrow of the same thing can be created or used (original one)
    // let immutable_borrowed_i = &mutable_i;
    // Either one mutable borrow can exist or multiple immutable borrow can exist
    // println!("Mutable borrowed i {}", mutable_borrowed_i);

    // mutable if we want to borrow and update the data and immutable if we want to only read it

    // --------- Constants ----------- //
    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing --> We can define a variable again with the same name and it will be shaddowed
    let spaces = "    ";
    let spaces = spaces.len();

    {
        // Twice the space in this block
        let spaces = spaces * 2;
        println!("Twice spaces {}", spaces);
    }

    println!("Spaces in input are {}", spaces);

    // -------- Control Flow ----------- //
    count_to_10_twice();
    count_to_10_while_loop();
    let arr = [2, 3, 4];
    print_element_of_int_array(&arr);

    // -------- Structs ------------- //
    let mut my_account: BankAccount = BankAccount {
        name: "Ayaan".to_string(),
        balance: 120.00,
    };

    my_account.withdraw(110.23223);
    my_account.print_balance();

    struct User {
        name: String,
        email: String,
        age: u32,
    }

    let user1 = User {
        name: String::from("Ayaan"),
        email: String::from("ayaan@example.com"),
        age: 20,
    };

    // We can create a new struct data with existing one
    let _user2 = User {
        name: String::from("John"),
        ..user1
    };

    // Tupple structs (without name for properties)
    struct Color(u32, u32, u32);
    let _my_color = Color(0, 0, 0);

    // Unit-Like structs --> nothing in them

    struct Yay;
    let _yay = Yay;

    // --------- Enums ------------- //
    // A type where we can select from possible variants

    enum IPAddrKind {
        V4,
        V6,
    }

    let my_ip = IPAddrKind::V4;

    // We can even add data to the enum
    enum IPAddrKindWithData {
        V4(String),
        V6(String),
    }

    let _my_ip_data = IPAddrKindWithData::V4(String::from("127.0.0.1"));

    // --------- Match --------------- //
    // Used to match various possible values for a given data
    // We can also used it in functions to create reuseable utils
    let res = match my_ip {
        IPAddrKind::V4 => String::from("IPV4 passed"),
        IPAddrKind::V6 => String::from("IPV6 passed"),
    };

    println!("Result for passed IP is {}", res);

    // --------- Error Handling --------- //
    // Rust have two built in ways for error handling
    // Using Result<T,E>

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    match divide(20.00, 0.00) {
        Err(err) => println!("{}", err),
        Ok(res) => println!("{}", res),
    };

    // We also have Option<T> --> Some() represents value
    fn divide_option(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 { None } else { Some(a / b) }
    }

    match divide_option(20.0, 0.0) {
        None => println!("Something went wrong!"),
        Some(val) => println!("{}", val),
    }

    // ------ Vectors --------- //
    // Dynamic, Homogenous collection of data
    let mut v: Vec<i32> = Vec::new();
    v.push(23);

    // We can also inittialize with some value too
    let my_v = vec![1, 2, 3];
    println!("{}", my_v[2]);

    // We can also use get
    match v.get(1) {
        Some(val) => println!("{}", val),
        None => println!("Nothing exist!"),
    }

    // ------- HashMap ------- //
    let mut points: HashMap<String, i32> = HashMap::new();

    points.insert(String::from("Ayaan"), 20);
    points.insert(String::from("John"), 25);

    let name = String::from("Ayaan");
    let point = points.get(&name).copied().unwrap_or(0);

    println!("Ayaan's point = {}", point);

    for (key, val) in points {
        println!("{key} {val}");
    }

    // let mut numbers = vec![1, 2, 3];
    // let first = &numbers[0];
    // numbers.push(4);
    // println!("{first}")
}

// functions --> Follows Hoisting
fn hello_world() {
    println!("Hello Functions")
}

fn area_of_rect(length: u32, width: u32) -> u32 {
    return length * width;
}

// Const --> Can't use mut keyword, explicitly type the type for this, can be defined in any scope
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

// Control flow
// Conditional
fn _check_age(age: i32) {
    if age > 18 {
        println!("Age is greater than 18");
    } else if age < 18 {
        println!("Age is less than 18");
    } else {
        println!("Age is 18");
    }
}

// Loops
fn count_to_10_twice() {
    // We can give labels to loops in order to break them in nested loop
    let mut count = 0;
    'counting_up: loop {
        let mut current_num = 1;
        println!("Counting till 10 {}", count + 1);
        loop {
            if current_num == 11 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            println!("{}", current_num);
            current_num += 1;
        }
        count += 1;
    }
}

fn count_to_10_while_loop() {
    let mut current_num = 1;
    while current_num < 11 {
        println!("{}", current_num);
        current_num += 1;
    }
}

fn print_element_of_int_array(arr: &[i32]) {
    for el in arr {
        println!("{}", *el);
    }
}

// Struct
// Group related data
struct BankAccount {
    name: String,
    balance: f64,
}

impl BankAccount {
    // Here we borrow data immutable as we won't mutate it
    fn print_balance(&self) {
        println!("Balance in {}'s account is {:.2}", self.name, self.balance);
    }

    // Here we borrow data mutably as we will mutate it
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing amount {:.2}", amount);
        if self.balance < amount {
            println!("Insufficient balance {}", self.balance);
        } else {
            self.balance -= amount;
            println!("Amount withdrawl successful");
        }
    }
}
