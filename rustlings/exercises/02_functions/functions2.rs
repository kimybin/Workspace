// TODO: Add the missing type of the argument `num` after the colon `:`.
// 매개변수는 타입 추론을 해주지 않기 때문에 타입을 반드시 명시 필요 
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
