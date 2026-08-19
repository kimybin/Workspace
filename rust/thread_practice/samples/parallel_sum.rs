// 스레드 여러 개 띄우고 결과 합치기
use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || { // ✅ 클로저가 move로 캡처하는 이유는? 
            println!("스레드 {} 시작", i);
            i * i // 결과값 리턴 
        });
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        let result = handle.join().unwrap(); // ✅ handle.join()이 왜 Result를 리턴하는지 (스레드가 패닉나면?)
        results.push(result);
    }

    println!("결과: {:?}", results);
}
