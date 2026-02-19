fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;
    //let v = vec![10, 20, 30, 40];

    //1과 2의 차이는? 
    //let v = vec![a[0], a[1], a[2], a[3]]; // 1. 배열의 요소를 하나하나 참조. 
    let v = Vec::from(a);  // 2. 배열이나 슬라이스를 벡터로 바로 변환할 때 가장 많이 쓰는 방식. 소유권 이동. 기존 a배열을 사용할 수 없게된다.
    //let v = a.to_vec();  //3. to_vec() 사용. 배열의 데이터를 복사하여 새로운 벡터를 만듬.

    (a, v)
}

//# 테스트 : 스트링 배열로 Vec::from(b); 해보기. 이동인가, 복사인가
// - 메모리를 새로 빌려서 써넣은 `String::from("apple")...` 게 아닌 이상 이 정도는 복제가 일어나서 이 테스트 의미 없음.
// &str(문자열 슬라이스 String Slice) 과 String(Owned String)
// 외부 데이터를 다를 때는 String을 사용. 그런데 함수의 인자를 설계할 땐 &str을 사용.

//fn array_string_to_vec() ->([&'static str; 3], Vec<&'static str>) {
/*
fn array_string_to_vec() ->([String; 3], Vec<String>) {

    //let t = ["apple", "banana", "carot"];
    let t = [
        String::from("apple"), 
        String::from("banana"), 
        String::from("carrot")
    ];

    let v = Vec::from(t);

    (t, v)
}
*/

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }

    // #[test]
    // fn test_array_string_to_vec() {
    //     let (t, v) = array_string_to_vec();
    //     assert_eq!(t, *v);
    // }

} 
