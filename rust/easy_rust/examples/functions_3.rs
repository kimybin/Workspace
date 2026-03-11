fn call_me(num: u8) { // u8은 부호 있는 정수 (음수, 0, 양수)
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(5);
}