// Calls of this function should be replaced with calls of `string_slice` or `string`.
// placeholder 함수호출하면, string_slice  또는  string   함수로 대체되어 호출되어야 한다
fn placeholder() {
    //직접 반환할 필요가 없지.
    //맞는 함수로 분기 해주면 되니까 이 함수에 반환값은 필요 없지. 
    // 분기하는 로직과 거기에 맞는 함수가 있으면 돼. 
    // ...이게 아니구나. 그냥 알아서 직접 호출 명령을 주면 된다. 
}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
// TODO: 여기에 여러 값들이 있습니다. 어떤 것은 `String`이고, 어떤 것은 `&str`이다.
// 과제는 각 값이 무엇인지 판단하여 `placeholder(…)`를 `string_slice(…)`
// 또는 `string(…)`으로 교체하는 것.

fn main() {
    //placeholder("blue");
    //변할게 없으니 기존 소유권 주소를 가리키면 된다. 
    string_slice("blue");

    //placeholder("red".to_string());
    //타입변환을 했다. string으로 바꿔으니 반환은 string인데. 
    //혹시 타입변환을 거치면 무조건 string으로 반환할까? 
    //왜냐면, 타입이 바뀌면 주소를 새로 써야하니까. 기존 주소에 타입 정보만 추가해서 처리할 순 없을 거 같은데.
    string("red".to_string());

    //placeholder(String::from("hi"));
    // String::from({}) 과 to_string 은 무슨 차이지?
    //  &str을 String으로 만들 때는 사실상 같다.
    //  String::from({})  : From이라는 트레이트(기능)를 사용한 방식
    //  to_string : ToString이라는 트레이트를 사용한 방식입니다. 내부적으로 결국 String::from을 호출
    // 현대 러스트에서는 성능 차이가 없음. 보통 String::from 이 생성하는 느낌이라 선호.
    string(String::from("hi"));

    //placeholder("rust is fun!".to_owned());
    //to_owned()는 빌림 한 것을 소유권이 있는 것으로 요청하는 거지? 그럼 반환이 String인거?
    //이를 "내 소유의 데이터"로 만들고 싶을 때 to_owned() 사용.
    string("rust is fun!".to_owned());
    
    //placeholder("nice weather".into());
    //into() 는 타입을 추론을 통해 input 타입으로 변환해주는 거야??
    //문서에 아래 처럼 써있다. 
    //fn into(self) -> T
    //Converts this type into the (usually inferred) input type.
    // 내가 가진 데이터를 호출된 함수가 원하는 타입으로 변환 시킨다.
    // 그래서 둘 다 가능하다.
    string_slice("nice weather".into()); 
    string("nice weather".into()); 

    //placeholder(format!("Interpolation {}", "Station"));
    //포맷을 바꾼다. 힙에 작성해야 겠지? 그래서 string
    string(format!("Interpolation {}", "Station"));
    
    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    // 경고: byte인덱싱은 문자열 인덱싱이 아님
    // 문자열 인덱싱은 `~` 을 사용할 수 있음
    //placeholder(&String::from("abc")[0..1]);
    //새로운 String을 생성했는데, 이걸 다시 슬라이싱하고 앞에 &를 붙였다. 
    string_slice(&String::from("abc")[0..1]);

    //placeholder("  hello there ".trim());
    //메모리 주소의 시작과 끝만 수정하면 되니까, 슬라이스로 처리
    string_slice("  hello there ".trim());

    //placeholder("Happy Monday!".replace("Mon", "Tues"));
    //값을 바꾸니까 기존 주소를 가리키면 안되겠지. 아예 소유권을 넘기자 
    string("Happy Monday!".replace("Mon", "Tues"));
    
    //placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
    //소문자로 바꾸기만 하면 메모리 주소만 바꿔주면 될까? 아닌가 보다.
    //유니코드에서는 대문자가 소문자로 바뀔 때 바이트 크기가 달라지는 경우가 있다
    //무조건 새로운 메모리 할당해야 한다.
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
