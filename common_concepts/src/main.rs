fn main() {
    // let x: i64 = 100;
    // x = 200; // error[E0384]: cannot assign twice to immutable variable `x`
    // println!("x is {}", x);

    let mut x: i64 = 100;
    x = 200;
    println!("x is {}", x); // will print 200

    const COUNT: i64 = 100_000;
    // can use _ for better readability
    // const cannot be mut
    // COUNT = 200; // error[E0070]: invalid left-hand side expression

    // Shadowing
    let var: i32 = 100;
    println!("var is {}", var); // prints 100
    let var: &str = "abc";
    println!("var is {}", var); // prints "abc"

    // Scalar data types
    // Integer types, Floating-point types, Boolean type, Character type

    // Compound data types
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup is {:?}", tup); // prints (500, 6.4, 1)
    let (x, y, z) = tup;
    println!("x is {}, y is {}, z is {}", x, y, z); // prints x is 500, y is 6.4, z is 1
    println!("tup.0 is {}", tup.0); // prints 500

    // Array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr is {:?}", arr); // prints [1, 2, 3, 4, 5]
    let arr: [i32; 5] = [1; 5];
    println!("arr is {:?}", arr); // prints [1, 1, 1, 1, 1]
    let arr = [1; 5];
    println!("arr is {:?}", arr); // prints [1, 1, 1, 1, 1]
    let arr = [1, 2, 3, 4, 5];
    println!("arr[0] is {}", arr[0]); // prints 1
                                      // println!("arr[10] is {}", arr[10]); // error[E0277]: the trait bound `[{integer}; 5]: std::ops::Index<{integer}>` is not satisfied

    // Functions
    println!("The sum of 1 and 2 is {}", sum(1, 2)); // prints The sum of 1 and 2 is 3

    // Control flow
    if 1 == 1 {
        println!("1 is equal to 1");
    } else if 1 == 2 {
        println!("1 is equal to 2");
    } else {
        println!("1 is not equal to 1");
    }

    let num: i64 = if 1 == 1 { 1 } else { 2 };
    println!("num is {}", num); // prints 1

    // loops
    let mut counter: i64 = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    let mut counter: i64 = 0;
    while counter < 10 {
        counter += 1;
    }

    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("element is {}", element);
    }

    for number in (1..10).rev() {
        // 10 is not included
        println!("number is {}", number);
    }
}

fn sum(x: i32, y: i32) -> i32 {
    // return type is i32
    x + y // same as "return x + y;", also remove semicolon
}
