fn fill_vec(vec: Vec<i32>) -> Vec<i32> {

    let mut vec = vec;

    vec.push(88);
    vec
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

        let temp_vec = vec0.clone();

        let vec1 = fill_vec(temp_vec);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

/*
# 문제 이해.
- vec0이 함수를 통과해 vec1이 된다. 이때 벡터에 88이 추가된다.
- 동시에 vec0은 기존값을 그대로 유지해야 한다. 

# 어떻게 해결할 것인가?
- 문제는 vec0의 인자로 사용되면서 소유권이 넘어가는 게 문제

# 1. 일단 빌리고, 이를 복사해서 사용하기
# 2. 빌려 줄 때 애초에 복사해서 인자로 보내기
# 3. 빌려 주기 전에 따로 복사하고, 그걸 인자로 보내기

*/