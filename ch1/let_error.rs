fn main() {
    let mut a = 100;
    // a = a + 1;  // 변수의 값을 변경할 수 없다!!: 러스트의 변수는 기본적으로 불변(Immutable)
    a = a + 1;
    println!("{}", a);
}