// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue"); // 리터럴 -> &str

    string("red".to_string()); // to_string(): &str -> 새 String 할당

    string(String::from("hi")); // String::from: 새 String 생성 (소유)

    string("rust is fun!".to_owned()); // to_owned(): 복사해서 String으로 소유권 획득

    string("nice weather".into()); // into(): 받는 쪽(string)이 String을 요구하므로 String으로 추론

    string(format!("Interpolation {}", "Station")); // format!: 새로 조합한 String 반환

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]); // 인덱싱은 복사 없이 원본 일부를 가리키는 &str

    string_slice("  hello there ".trim()); // trim(): 원본을 가리키는 슬라이스(&str) 반환, 새 할당 없음

    string("Happy Monday!".replace("Mon", "Tues")); // replace(): 치환된 새 String 생성

    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // to_lowercase(): 변환된 새 String 생성
}
