use std::thread; 
use rand::Rng;

fn main() {
    let mut handles = vec![]; // 스레드 모아두기 

    for i in 0..5 { // 스레드 5개 만들기 
        // 클로저는 별도 스레드에서 돌고, 그 결과를 메인 쪽으로 가져오는 유일한 통로는 
        // 클로저의 리턴값을 handle.join()으로 받는 것뿐
        // move로 옮기고 그 안에서 계산한 값을 클로저 리턴값으로 내보내줘야 함
        let handle = thread::spawn(move || {
            let mut sum = 0; 
            
            for j in 0..10 { // 10개 뽑아서 합 계산 
                let random = rand::thread_rng().gen_range(0..99); // 랜덤 정수 생성 
                sum += random; // 합 계산  
            }
            sum  // 진짜 리턴값
        });
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        let result = handle.join().unwrap(); // join()으로 모으기 
        results.push(result);
    }

    println!("{:?}", results);

}