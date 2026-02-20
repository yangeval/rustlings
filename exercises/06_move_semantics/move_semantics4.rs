fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();

        let y = &mut x;
        y.push(42);

        let z = &mut x;
        z.push(13);

        assert_eq!(x, [42, 13]);
    }
}

/* 
# 문제
- 대여 규칙: 가변참조자의 제약
- 소유권을 넘긴 게 아님. 
- 가변 참조자(&mut)가 있으면, 그 시점에 한 명만 값을 보거나 수정할 수 있다. 
- 비유: 동시에 한 개의 물건을 고치려는 것.

# 해결
- 동시에 x에 접근하지 않게 순서를 변경
*/