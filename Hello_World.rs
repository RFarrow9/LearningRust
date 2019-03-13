use std::fmt::{self, Formatter, Display};

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
    println!("");
    //write!(f, "{}", value)?;

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            //Iterate over vec in v while enumerating the iteration
            //count in 'count'
            for (count, v) in vec.iter().enumerate() {
                // Use the ? operator, or try!(deprecated) to return errors
                if count != 0 {write!(f, ", ")?;}
                write!(f, "{}", v)?;
            }
            //Close the opened bracket and return the fmt::result
            write!(f, "]")
        }
    }

    let m = List(vec![1, 2, 24]);
    println!("{}", m);
    let foo :i64 = 3735928559;
    println!("{}", format!("{}", foo));
    println!("{}", format!("0x{:X}", foo));
    println!("{}", format!("0o{:o}", foo));
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }
    impl Display for City{
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
            let lon_c = if self.lon >= 0.0 {'E'} else {'W'};
            write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }
    for city in [
        City {name: "Dublin", lat: 53.347778, lon: -6.259722},
        City {name: "Oslo", lat: 59.95, lon: 10.75},
        City {name: "Vancouver", lat: 49.25, lon: -123.1},
        ].iter() {println!("{}", city);}

    #[derive(Debug)]
    struct Colour {
        red: u8,
        green: u8,
        blue: u8,
    }
    impl Display for Colour {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "RGB:({},{},{})", self.red, self.green, self.blue)
        }
    }
    for colour in [
        Colour {red: 43, green: 58, blue: 12},
        Colour {red: 1, green: 255, blue: 125},
        Colour {red: 95, green: 47, blue: 255},
    ].iter() {println!("{}", colour);}
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