fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let (s4, len) = calculate_length(s3);
    println!("The length of '{}' is {}.", s4, len);

    let len = cal_length_using_add(&s4);
    println!("The length is : {} ", len);
}

fn cal_length_using_add(s: &String) -> usize {
    s.len()
}

fn calculate_length(s:String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String {
    a_string
}
