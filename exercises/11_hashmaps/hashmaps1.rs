// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.
//
// 해시 맵 형태의 과일 바구니를 정의해야 합니다. 키는 과일의 이름을 나타내고,
// 값은 바구니에 담긴 해당 과일의 개수를 나타냅니다. 바구니에는 최소 3가지
// 이상의 서로 다른 종류의 과일(예: 사과, 바나나, 망고)을 넣어야 하며,
// 모든 과일의 총 개수는 최소 5개 이상이어야 합니다.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    // let mut basket =

    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("사과"), 3);
    basket.insert(String::from("망고"), 77);
    
    // TODO: Put more fruits in your basket.
    
    
    basket
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);

        //println!("{}", basket.values().sum::<u32>());
        //assert!(false);
    }
}
