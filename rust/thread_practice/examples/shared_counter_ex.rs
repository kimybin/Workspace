use std::thread;
use rand::Rng; 
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0)); // lock 걸 수 있는 공유 변수 만들기 
    let mut handles = vec![];

    for i in 0..5 { 
        let counter = Arc::clone(&counter); // Arc::clone으로 여러 스레드가 같은 Mutex를 가리키는 포인터 나눠갖음 

        let handle = thread::spawn(move || {
            let mut sum = 0; 
            for j in 0..10 { 
                let random = rand::thread_rng().gen_range(0..99);
                sum += random; 
            }
            *counter.lock().unwrap() += sum; // lock()을 잡는 것 = "지금부터 내가 이 공유 데이터 쓸 거니까 다른 스레드는 기다려" 의미 
        }); // 다 쓰면(가드가 스코프 벗어나면) 자동으로 unlock
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); 
    }
    println!("{:?}", *counter.lock().unwrap()); // 새로 lock() 걸어서 새 가드를 만들기 -> 값 출력 -> 문장 끝나면 그 가드도 바로 drop

}