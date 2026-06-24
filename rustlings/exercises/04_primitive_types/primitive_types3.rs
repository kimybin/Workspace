fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [0; 100]; // 같은 값 0을 100개 채운 배열 

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");   
    }
}
