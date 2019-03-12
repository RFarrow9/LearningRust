use std::fmt;

// Rust has to have a main function
fn main() {

    let answer :i32 = 43;
    println!("Hello {}", answer);
    assert_eq!(answer, 43);
    for i in 0..7 {
        //for printing the {} will be replaced by subsequent parameters
        println!("Hello {}", i)
    }
    //checking conditionals
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
    for i in 0..5 {
        let even_odd :&str = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }
    println!("test {} {}", 1, 1);
    println!("{1}, {0}", "jim", "dwight");
    //formats the number as a binary string
    println!("{} of {:b} people know binary, the other half doesn't", 1, 226);
    let mut sum :f64 = 0.0f64;
    for i in 0i32..47i32 {
        sum += i as f64;
    }
    println!("Sum is {}", sum);
    //all variables must be created using the let keyword
    //variables are by default read only, not mutable!!
    //nonmut = 31;
    //println!("sum is {}", nonmut);

    let res :f64 = sqr(2.87f64);
    println!("{}", res);

    // Define a structure which `fmt::Display` will be implemented for. This is simply
    // a tuple struct containing an `i32` bound to the name `Structure`.
    struct Structure(i32);

    // In order to use the `{}` marker, the trait `fmt::Display` must be implemented
    // manually for the type.
    impl fmt::Display for Structure {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }
    let test = Structure(45);
    println!("{}", test);

}

//In functions, the compiler will not infer types
fn sqr(x: f64) -> f64 {
    return x * x;
}
