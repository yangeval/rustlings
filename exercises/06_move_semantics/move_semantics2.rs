fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    //let mut vec = vec.clone();
    //vec.push(88);
    
    //# 1
    //test에서 vec0 을 clone() 해서 복사본을 인자로 전달
    //매개변수에서 mut로 받아서 추가해서 반환
    /*
    vec.push(88);
    vec
    */

    //# 2
    // 빌려서 처리. 매개변수에 & 추가
    let mut new_vec = vec.clone(); //빌려온 걸 새 변수에 복사해서 할당
    new_vec.push(88);
    new_vec

}
 
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        //let vec1 = fill_vec(vec0);
        //let vec1 = fill_vec(vec0.clone()); //1번
        let vec1 = fill_vec(&vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
