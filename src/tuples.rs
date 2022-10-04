// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let tup1 = (20, 25, 30, 35);
    println!("{}", tup1.2);

    // different types
    let tup2 = (20, "Rust", 3.4, false);
    println!("{}", tup2.2);

    // tuple in tuple, nested tuple
    let tup3 = (20, "Rust", 3.4, (1, 2, 5));
    println!("{}", (tup3.3).0);

    //
    let tup4 = (1, 9, "Rust");
    let (a, b, c) = tup4;

    println!("{} {} {}", a, b, c);
}
