use std::fmt;

// Rust has to have a main function
fn main() {
    // Define a structure which `fmt::Display` will be implemented for. This is simply
    // a tuple struct containing an `i32` bound to the name `Structure`.
    // This won't print by default, as the 'print method' needs to be defined, similar to the
    // __str__ magic method of python.
    #[allow(dead_code)]
    struct Structure(i32);
    impl fmt::Display for Structure {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    struct MinMax(i32, i32);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Min: {0}  Max: {1}", self.0, self.1)
        }
    }
    let minmax = MinMax(5, 12);
    println!("{}", minmax);
    //let test = Structure(45);
    //println!("{}", test);

    #[derive(Debug)]
    struct Point2d {
        x: f64,
        y: f64
        //Interestingly delimiter required on the last entry
    }
    let newpoint = Point2d{x: 21.4, y: 48.3};
    impl fmt::Display for Point2d {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {} y: {}", self.x, self.y)
        }
    }
    println!("{}", newpoint);
}

//In functions, the compiler will not infer types
fn sqr(x: f64) -> f64 {
    return x * x;
}

#[allow(dead_code)]
fn archive() {
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

}