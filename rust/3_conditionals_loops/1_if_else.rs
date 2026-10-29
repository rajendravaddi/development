fn main(){
    /*
    we have `if`, `else` and `else if` statements
    */
    // Example 1 - Grading

    let score = 85;

    if score >= 90 {
    println!("Grade: A");
    } else if score >= 80 {
    println!("Grade: B");
    } else if score >= 70 {
    println!("Grade: C");
    } else {
    println!("Grade: F");
    }

    // Example 2 - Even and less than 10
    let x = 10;
    // parantheses around condition is unnecessary - it will raise warning
    if x%2==0 && x<10{
        println!("{} is even and less than 10",x);
    }
    else if x%2!=0 && x<10{
        println!("{} is odd and less than 10",x);
    }
    else if x%2==0 && x>=10{
        println!("{} is even and greater than or equal to 10",x);
    }
    else{
        println!("{} is odd and greater than or equal to 10",x);
    }

    // Using if as expression
    // Here we should not mix types , the return values in if and else should be of same type
    let x = 10;
    let a = if x>10{
        "Greater than 10"
    }else{
        "less than or equal to 10"
    };
    println!("{}",a);

}