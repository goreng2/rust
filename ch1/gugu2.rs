fn main() {
    // 이해는 안되지만.. Python의 List Comprehension을 Rust에서도 구현..
    for y in 1 .. 9 {
        let s = (1 .. 10)
                .map(|x| format!("{:3}", x * y))
                .collect::<Vec<String>>().join(",");
        println!("{}", s);
    }
}