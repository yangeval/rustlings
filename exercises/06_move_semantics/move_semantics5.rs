#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).
// & 기호를 넣거나 빼서 컴파일 에러를 고쳐

// Shouldn't take ownership
//fn get_char(data: &String) -> char {
fn get_char(data: &String){
    //&를 사용하지 않으면, 소유권을 가져온다. 
    //-main의 이후 함수에서 data를 사용할 수 없게 된다. 
    //data.chars().last().unwrap()
    let last_char = data.chars().last().unwrap();
    println!("{}", last_char);
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
