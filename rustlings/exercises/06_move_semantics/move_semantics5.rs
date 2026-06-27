#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).
// 이 함수가 데이터를 잠깐 들여다보기만 하면 &를 붙이고, 데이터를 진짜로 가져다 바꾸거나 저장하면 &를 뗌

// Shouldn't take ownership
// -> 글자 하나만 보면 되니까 "빌리기"만 하면 충분 -> &String으로 받음
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// -> 대문자로 바꾼 새 값을 만들어 쓰는 거니까 진짜로 "소유"해야 함 -> String으로 받음
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    // to_uppercase()는 String을 반환하므로 data도 String이어야 타입이 맞음

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    // & 추가: get_char는 빌리기만 하니까 data의 소유권은 안 넘어감
    get_char(&data);

    // & 없음: string_uppercase는 소유권을 가져가야 하니까 data 자체를 넘김 (마지막 사용)
    string_uppercase(data);
}
