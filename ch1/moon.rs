fn main() {
    let to_the_moon = 384400.0;  // 실수를 표현하기 위해 .0을 붙인다.
    println!("지구에서 달까지 차로는 {}일, 기차로는 {}일 걸린다", (to_the_moon / 80.0) / 24.0, (to_the_moon / 300.0) / 24.0);
}