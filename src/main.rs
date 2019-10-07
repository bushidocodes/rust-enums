fn main() {
    // enums = enumerates possible variants
    // Similar to algebraic data types in function languages, such as F#, OCaml, and Haskell

    // Compare booleans to enums

    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    };

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));


    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // // Can take any of the IpAddrKind variants
    // fn route(ip_type: IpAddrKind) {}

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // We can group different variants under a single message type. This allows us to accept the different variants as parameters
    #[derive(Debug)]
    enum Message {
        Quit,                       // No Data Associated
        Move { x: i32, y: i32 },    // Anonymous struct
        Write(String),              // Single String
        ChangeColor(i32, i32, i32), // 3-tuple of 32 bit ints
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("Quitting!"),
                Message::Write(text) => println!("{}", text),
                _ => (),
            }
            // println!("{}", self.0);
        }
    }

    let m = Message::Write(String::from("YOLO"));
    m.call();

    enum Shape {
        Circle { radius: f64 },
        Rectangle { height: f64, width: f64 },
    }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
                Shape::Rectangle { height, width } => height * width,
            }
        }
    }

    let my_rectangle = Shape::Rectangle {
        height: 32f64,
        width: 20f64,
    };
    println!("My Rectangle has an area of {}", my_rectangle.area());

    let my_circle = Shape::Circle { radius: 5.4 };
    println!("My circle has an area of {}", my_circle.area());


    println!("Hello, world!");

    // Use of Option enum in place of null
    // Included in the prelude
    // Generic Type Parameter

    // Some can infer the type
    let some_number = Some(5);
    // But None has nothing to infer against, so we need to manually set the type parameter
    let no_number: Option<i32> = None;

    let some_string = Some("a string!");
    let no_string: Option<&str> = None;

    // The language doesn't auto-unpack some. The programmer needs to do this via a match, and the compiler enforces coverage across all possible variants

    // let sum = some_number + 5;

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // Coin::Quarter => 25,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    println!("{}", value_in_cents(Coin::Quarter(UsState::Connecticut)));

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        Arizona,
        Arkansas,
        California,
        Colorado,
        Connecticut,
        Delaware,
        DistrictOfColumbia,
        Florida,
        Georgia,
        Guam,
        Hawaii,
        Idaho,
        Illinois,
        Indiana,
        Iowa,
        Kansas,
        Kentucky,
        Louisiana,
        Maine,
        Maryland,
        Massachusetts,
        Michigan,
        Minnesota,
        Mississippi,
        Missouri,
        Montana,
        Nebraska,
        Nevada,
        NewHampshire,
        NewJersey,
        NewMexico,
        NewYork,
        NorthCarolina,
        NorthDakota,
        Ohio,
        Oklahoma,
        Oregon,
        Pennsylvania,
        RhodeIsland,
        SouthCarolina,
        SouthDakota,
        Tennessee,
        Texas,
        Utah,
        Vermont,
        Virginia,
        Washington,
        WestVirginia,
        Wisconsin,
        Wyoming,
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 7u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(42u8);

    if let Some(42) = some_u8_value {
        println!(
            "Behold the Answer to the Ultimate Question of Life, the Universe, and Everything"
        );
    } else {
        println!("As you were!");
    }

}
