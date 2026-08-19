// Mutex로 공유 카운터 만들기
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // race condition 막기 
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // 포인터로 접근 권한 풀어주기 
        let handle = thread::spawn(move || { // 새로운 OS 스레드 만들고, 코드 실행
            let mut num = counter.lock().unwrap(); // drop되는 순간 자동 lock 해제
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles { // 스레드가 완전히 끝날 때까지, 메인 스레드에서 대기시키기 
        handle.join().unwrap();
    }

    println!("최종 카운터 값: {}", *counter.lock().unwrap()); // 최종 값 확인을 위해, 다시 lock 잡고+역참조
}