### 실습) 스레드 여러 개 띄우고 결과 합치기


```rust
use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || { // ✅ 1. 클로저가 move로 캡처하는 이유는? 
            println!("스레드 {} 시작", i);
            i * i // 결과값 리턴 
        });
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        let result = handle.join().unwrap(); // ✅ 2. handle.join()이 왜 Result를 리턴하는지 (스레드가 패닉나면?)
        results.push(result);
    }

    println!("결과: {:?}", results);
}

```
---

### 트러블 슈팅)

1. 왜 move가 필요할까?
- Rust 클로저는 기본적으로 변수를 "빌려서(borrow)" 캡처하려고 함 -> 🌟 **즉 원본 소유권은 그대로 두고 참조만 가져가는 것**

- 근데 스레드는 요상한 문제가 있음: **메인 함수가 언제 끝날지, 스레드가 언제 끝날지 컴파일 타임에 보장할 수 없다는 것 ❌ ** 

- 만약 클로저가 i를 참조로만 가져갔는데, 메인 스레드가 먼저 끝나서 i가 스택에서 사라져버리면? 
    - **⚠️ 스레드는 이미 없어진 메모리를 가리키게 됨** — 이게 바로 C/C++에서 흔한 **⚠️ dangling reference 버그**
<br>
- Rust는 이걸 컴파일 타임에 막아버림 
    - -> 🌟 **move를 쓰면 "이 클로저가 캡처하는 값들의 소유권을 통째로 넘겨줘"라는 뜻**이 되기 때문에 그럼 스레드가 그 값을 온전히 소유하니까, 원본이 언제 사라지든 상관없이 안전함 ⭕️

<br>

2. 왜 join()이 Result를 리턴할까?
- 스레드 안에서 ⚠️ 패닉(panic)이 날 수가 있음 — 배열 범위 초과, unwrap() 실패 등. 
- Rust에서는 특별히 ⚠️ 이 에러를 명시적으로 다루게 강제함:
    - 스레드가 정상 종료 → Ok(반환값)
    - 스레드가 패닉으로 죽음 → Err(패닉 정보)
- 그래서 handle.join().unwrap()을 쓰면 "패닉났으면 여기서도 같이 패닉내" 라는 뜻이고, 
- 진짜 프로덕션 코드라면 match로 Err 케이스를 잡아서 로그 남기고 넘어가는 식으로 처리해야 함
- Rust는 그걸 Result 타입 하나로 통일해서 표현