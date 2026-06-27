// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // 섀도잉(같은 이름으로 재바인딩)으로 mut를 붙인 것. 이건 borrow 문제 해결이 아니라
    // 소유권을 이미 넘겨받은 값(vec)을 가변으로 다시 바인딩하는 것뿐.
    // 빌린(참조) 값은 섀도잉으로 못 바꿈 -> 수정하려면 &mut로 빌려야 함 (move_semantics2 참고)
    let mut vec = vec;

    vec.push(88);

    vec
} 

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
