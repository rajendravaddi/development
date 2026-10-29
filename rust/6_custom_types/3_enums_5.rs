#![allow(dead_code)]

enum Branch {
    CSE,
    ECE,
    MECH,
    CIVIL,
    EEE,
}

enum CourseYear {
    E1,
    E2,
    E3,
    E4,
}
fn main() {
    /*
    The `use` declaration can be used to avoid typing the full module path to access a name.
    */

    use Branch :: {CSE, ECE, CIVIL, MECH, EEE}; // specified only
    use CourseYear :: *; // everything from enum

    let b = CSE;
    let y = E1;

    match b {
        CSE => println!("CSE"),
        ECE => println!("ECE"),
        MECH => println!("MECH"),
        CIVIL => println!("CIVIL"),
        EEE => println!("EEE"),
    }

    match y {
        E1 => println!("E1"),
        E2 => println!("E2"),
        E3 => println!("E3"),
        E4 => println!("E4"),

    }
}