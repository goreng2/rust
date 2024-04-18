fn main() {
    // 변수 정의: 값이 변하므로 'mut' 추가
    let mut a = 1;
    let mut b = 1;
    println!("{}", a);
    println!("{}", b);
    for _ in 0..30 {
        println!("{}", a + b);
        let tmp = a;  // 값이 변하지 않으므로 일반 정의
        a = b;
        b = tmp + b;
    }
}