// 곱셈 함수 정의
fn multiplication(a: i64, b: i64) -> i64 {
    // 세미콜론을 붙이지 않으면 return을 안써도 된다.
    a * b
    // return a * b;
}

fn main() {
    let ex1 = multiplication(3, 5);
    println!("3*5={}", ex1);
    
    let ex2 = multiplication(8, 4);
    println!("8*4={}", ex2);
}