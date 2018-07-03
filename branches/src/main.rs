fn main() {
    // If
    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // While
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // For
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}