fn main() {
    let price = 3950;
    for i500 in 0..11 {
        for i100 in 0..4 {
            for i50 in 0..10 {
                let total = i500 * 500 + i100 * 100 + i50 * 50;
                if total == price {
                    println!("500원 {}개, 100원 {}개, 50원 {}개", i500, i100, i50);
                }
            }
        }
    }
}