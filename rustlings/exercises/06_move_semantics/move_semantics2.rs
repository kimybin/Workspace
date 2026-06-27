// & / &mut는 타입 앞에 붙는 참조 기호: Vec<i32>(소유), &Vec<i32>(불변 참조), &mut Vec<i32>(가변 참조)
// 매개변수 타입에 &가 없으면(예: 원래 코드 Vec<i32>) 호출자의 소유권이 함수로 이동(move)됨 -> 호출자는 이후 못 씀
// &mut로 받으면 그냥 "빌리는 것"이라 소유권은 호출자 쪽에 그대로 남음
// (구분법: 타입 앞에 &가 없으면 그 쪽이 owner, &가 있으면 그 쪽은 borrower)
// 참조로 받은 값은 통째로 반환 불가 (cannot move out of a mutable reference) -> 반환 타입 없이 제자리 수정만
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(88);
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // vec0과 별개의 데이터를 갖는 vec1을 만들려고 clone (참조를 빌려주는 것과는 다름)
        // 이름 있는 변수(vec1)를 먼저 만든 뒤 그 변수를 &mut로 빌려줘야 수정이 남음
        // (vec0.clone()을 그 자리에서 바로 빌려주면 임시값이라 그 줄이 끝나면 바로 drop됨)
        let mut vec1 = vec0.clone();
        fill_vec(&mut vec1);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
