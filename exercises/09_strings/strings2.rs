// TODO: Fix the compiler error in the `main` function without changing this function.
// main함수에서 컴파일 에러 고쳐. 이 함수는 수정하지 말고
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
// is_a_color_word함수의 매개변수는 빌려온 것이다. 
// 그러므로 함수를 사용하는 if문에서 빌려오든지, 
// 아니면, 변수 자체를 생성할 때 빌린 것으로 처리해야 한다.
// 그런데 문제인 이 코드를 수정하지 말라고 돼있다. 