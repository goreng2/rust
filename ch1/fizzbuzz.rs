fn main() {
    // 1~100 반복
    for i in 1 .. 101 {
        // 조건문
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i)
        }
    }
}