fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: Multiply each element in the `input` slice by 2 and push it to
        // the `output` vector.
        // 각 요소에 2의 배수해서 output에 넣기
        output.push(element *2);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // An example of collecting a vector after mapping.
    // We map each element of the `input` slice to its value plus 1.
    // If the input is `[1, 2, 3]`, the output is `[2, 3, 4]`.
    //아름답구만
    // input 매개변수로 들어오는 묶음(슬라이스)
    // .iter() 반복자 소환 -> 묶음에서 꺼낼 쓸 준비 완료(아직 꺼내지 않음)
    // .map(...) 가공 명령. 반복자에게 하나씩 꺼낼 때마다 ~이걸 해줘
    // |element| element + 1  : $f(x) = x + 1$ 이거와 같음. 
    //  꺼낸 녀석이 element  여기에 1을 더한다. 
    // .collection()  map() 만 했을 때는 아무일도 일어나지 않음. 
    //  이걸 호출하는 순간 하나씩 꺼내서 1을 더하고, 새 그릇에 담는다. 
    // 반환 타입을 보고 벡터 상자에 담는다.
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // 이 함수는 위의 vec_loop 와 기능상 동일한 입력과 출력을 만든다. 
    //그러나 이 방식을 더 많이 쓴다. 
    //함수형 방식이다. 
    // TODO: Here, we also want to multiply each element in the `input` slice
    // by 2, but with iterator mapping instead of manually pushing into an empty
    // vector.
    // See the example in the function `vec_map_example` above.
    
    input
        .iter()
        .map(|element| {
            element *2 
        })
        .collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
