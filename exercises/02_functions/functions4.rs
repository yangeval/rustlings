// #반환 값을 갖는 함수

// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// 짝수는 10, 홀수는 3 할인
// Don't worry about the function bodies themselves, we are only interested in
// the signatures for now.
// 반환에도...

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Fix the function signature.
fn sale_price(price: i64) -> i64{
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
