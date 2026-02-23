// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.
// 모듈 경로를 use, as 키워드 사용해서 스코프 안으로 옮길 수 있다.

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;
    
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}



fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
