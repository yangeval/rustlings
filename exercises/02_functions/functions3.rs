//fn call_me() {
    //    let num = 5;
    
fn call_me(num: u8) {  // type: unsigned 8bit
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(5);
}
