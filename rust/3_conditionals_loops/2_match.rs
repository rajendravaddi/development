fn main(){
    //Each part of the match branches must be the same type - just like with if...else.

    // Example 1
    let day = 1;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    // Example 2
    let day = 6;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    // Example 3 - ranges
    // 1..4 => 1,2,3
    // 1..=4 => 1,2,3,4
    let num = 1;

    match num {
        1..4 => println!("1 to 3"),
        // 2 => println!("Two"), // Here 2 already included in 1..4 , so it will give a warning while compilation
        // 3 => println!("Three"), // Here 3 already included in 1..4 , so it will give a warning while compilation
        5..=10 => println!("5 to 10"), // Range
        _ => println!("Something else"),
    }

    // Example 4 - destructuring
    let point = (1, 0);

    match point {
        (0, 0) => println!("At the origin"),
        (x, 0) => println!("On the x-axis at {}", x),
        (0, y) => println!("On the y-axis at {}", y),
        (x, y) => println!("At ({}, {})", x, y),
    }

    // Example 5 - Match with return
    let day = 1;

    let a = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day.",
    };

    println!("a is {}", a);
}