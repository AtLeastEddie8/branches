fn main() {
    let number = 8;
    if number < 5 {
        println!("condition was true");
    }
    else{
        println!("condition was false");
    }
    let num: bool = true;

    if num{
        println!("num is bool");
    }

    if number != 0{
        println!("number was not zero")
    }
    //Handling Multiple Conditions with else if
    let number = 12;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    //Using if in a let Statement
    let condition = true;
    let number = if condition { 9 } else { 2 };

    println!("The value of number is: {number}");
    //infinte loop
    // loop{
    //     println!("||||||||||||||||||||||||||||||||||||||||||||||||||||||");
    // }

    let mut counter1 = 0;

    let result = loop {
        counter1 += 1;

        if counter1 == 10 {
            break counter1 * 2;
        }
    };
    println!("result = {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    //Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    for re in 0..4{
        println!("The number IS : {re}")
    }
    //its the exact oppsite like 0-3 but instead 3-0 
    for re in (0..4).rev(){
        println!("The number rev IS : {re}")
    }


}
