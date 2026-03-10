fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        // 이 코드를 값이 Some일 때만 실행되는 if let 구문으로 바꾸세요.
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
        // word = optional_target {
        //     assert_eq!(word, target);
        // }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        // 할 일: 이 부분 코드를 while-let 구문으로 만드세요.
        // `Vec::pop()`은 값을 반환할 때 Option 상자를 한 겹 더 씌운다는(layer) 점을 기억하세요.
        // if-let 이나 while-let 구문 안에서 패턴 매칭(상자 까기)을 여러 겹 중첩해서 할 수 있습니다.
        
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }


        // integer = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }

        assert_eq!(cursor, 0);
    }
}
