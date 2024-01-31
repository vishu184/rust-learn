fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function(){
    println!("Inside another function");
    another_function2(2,'a');
}

fn another_function2(value : i32, unit_label : char){

    let y = {
        let x = 3;
        x+1
    };

    println!(" Y : {y}");
    println!("Value : {value} and unit label : {unit_label}");

    let y = number();

    println!("The value of Y : {y}");
}

fn number() -> i32 {
    4
}
