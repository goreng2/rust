fn main() {
    let n = 5;
    // 삼항연산자지만 결국은 변수 선언이므로 세미콜론이 맨 뒤에 붙는다.
    let check_even_odd = if n % 2 == 0 { "짝수" } else { "홀수" };
    println!("{}", check_even_odd);
}