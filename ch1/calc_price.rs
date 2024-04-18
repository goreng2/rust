fn main() {
    let pc_price = 980000;
    let pc_price_a = pc_price * 80/100 + 12000;
    let pc_price_b = pc_price * 90/100;

    println!("A: {}원, B: {}원", pc_price_a, pc_price_b);
    if pc_price_a > pc_price_b {
        println!("B가 더 싸다.");
    } else if pc_price_a < pc_price_b{
        println!("A가 더 싸다.");
    } else {
        println!("같다.");
    }
}