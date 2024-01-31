fn main() {
    
    let number = 3;

    if number < 3 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 and 2");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter==10 {
            break counter*2;
        }
    };

    println!("The result is : {result}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10,20,30,40,50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..5).rev() {
        println!("{number}");
    }

    let n = 6;

    let mut first = 0;
    let mut second = 1;
    let mut i = 2;
    let mut feb_num: i128 = 0;

    while i!=n && n>2{
        feb_num = first + second;
        first = second;
        second = feb_num;
        i = i+1;
    }

    println!("Nth Feb Number: {feb_num}");

}
