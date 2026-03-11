fn take_fifth(value: Vec<i32>) -> Option<i32> { // ✅ None or i32
    if value.len() < 5 { 
		    // .len() gives the length of the vec.
		    // It must be at least 5.
        None // // "값 없음"을 안전하게 표현 -> 패닉 방지 
    } else {
        Some(value[4])
    }
}

fn main() { 
    let new_vec = vec![1, 2]; // ✅ None return
    let bigger_vec = vec![1, 2, 3, 4, 5]; // ✅ Some(5) return 
    println!("{:?}, {:?}", take_fifth(new_vec), take_fifth(bigger_vec));
}

// ===== Option<T> 형식 ======
// enum Option<T> { // Generic
//			None, // variant
//			Some(T),			
// }