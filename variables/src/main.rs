fn main() {
    let x: i32 = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");

    let x = 2.0; // will be f64 the arch of CPU - as no explicit type given

    let y: f32 = 3.0; // will be f32 as explicit type given

    println!("x: {x}, y: {y}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // will give -1

    // remainder, properly called modulo
    let remainder = 43 % 5;

    // boolean
    let tr_ue = true; // with implicit type
    let fal_se: bool = false; // with explicit type

    // char types are 4 bytes in size and represent a Unicode Scalar Value
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type
    let heart_eyed_cat = '😻';

    // printing the values so compiler doesn't complain about unused variables
    println!("x: {x}, sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, truncated: {truncated}, remainder: {remainder}, tr_ue: {tr_ue}, fal_se: {fal_se}, c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // destructuring the tuple

    println!("x: {x}, y: {y}, z: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1); // tuples can hold different data types

    let five_hundred = x.0;   // accessing the first element of the tuple and
    let six_point_four = x.1; // the rest, by tuple index number, using a period
    let one = x.2;            // followed by the index number on the variable

    print!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");

    // array
    let a = [1, 2, 3, 4, 5]; // arrays must contain the same data types

    println!("a0: {}, a1: {}, a2: {}, a3: {}, a4: {}",
        a[0], a[1], a[2], a[3], a[4]);

    // arrays are useful when you want your data allocated on the stack, not
    // the heap, and you know the size of the array at compile time
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("month 1: {}, month 2: {}, month 3: {}, month 4: {}, month 5: {}, month 6: {}, month 7: {}, month 8: {}, month 9: {}, month 10: {}, month 11: {}, month 12: {}", months[0], months[1], months[2], months[3], months[4], months[5], months[6], months[7], months[8], months[9], months[10], months[11]);

    // here we are specifying the type and length of the array [i32; 5]
    // and then giving it our actual values to store in the array
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 5]; // this will create an array that contains 5 elements
                    // each with the value of 3
    println!("b0: {}, b1: {}, b2: {}, b3: {}, b4: {}",
        b[0], b[1], b[2], b[3], b[4]);

    // just like tuples, we can destructure arrays
    let first = a[0]; // accessing the first element of the array [1]
    let second = a[1]; // accessing the second element of the array [2]

    // both the tuple and the array structures are zero-indexed and (start at 0)
    // and the length of the tuple or array is fixed, meaning it cannot grow or
    // shrink. If you try and access an index that is out of range, your program
    // will panic at runtime!

    println!("first: {first}, second: {second}");

    let t = ([1; 2], [3; 4]);

    let (a, _b) = t;

    println!("{}", a[0] + t.1[0]);

}
