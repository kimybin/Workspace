// TODO: Add some function with the name `call_me` without arguments or a return value.
fn call_me() {

}
fn main() {
    call_me(); // Don't change this line
}

// example

// a, b가 매개변수 (둘 다 있어야 함)
// -> i32는 함수 리턴 타입 
// 만약 타입이 안 맞거나(-> i32인데 bool을 반환하는 등), 
// 반환 타입을 선언했는데 마지막에 세미콜론(a + b;)을 붙여서 ()(unit, 빈 값)를 반환하게 되면 컴파일 에러 발생 
// fn add(a: i32, b: i32) -> i32 { 
//     // a, b가 매개변수 (둘 다 있어야 함)
//     a + b
// }

// fn main() {
//     add(3, 5);  // 3, 5가 인자
// }