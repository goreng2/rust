fn main() {
    // 거스름 돈
    let price: i64 = 3950;

    // 내가 갖고있는 동전 개수
    // 반드시 계산하는 변수의 타입이 일치해야 한다.
    // price를 i64로 정의했으니, count000도 i64로 지정한다
    let count500: i64 = 10;
    let count100: i64 = 3;
    let count50: i64 = 10;

    // 거스름돈 조합 계산
    for i500 in 0..(count500+1) {
        for i100 in 0..(count100+1) {
            for i50 in 0..(count50+1) {
                let total: i64 = i500 * 500 + i100 * 100 + i50 *50;
                if price == total {
                    println!("500원 {}개, 100원 {}개, 50원 {}개", i500, i100, i50);
                }
            }
        }
    }
}