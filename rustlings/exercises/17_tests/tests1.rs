// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
} // 짝수이면 

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    // 상위 모듈에 정의된 것이라도 자식 모듈 안에서 자동으로 보이지 않고, use 로 명시적으로 가져와야 함!
    use super::*; 

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        // assert! !=  assert_eq! 
        // 인자를 하나만 받음 
        // 만약 assert!(is_even(2), true)처럼 두 번째 인자를 넣으면 그건 "조건이 실패했을 때 출력할 메시지"로 해석되는데 
        // true는 문자열이 아니라서 컴파일 에러 발생하게 됨 
        assert!(is_even(2));
        assert!(!is_even(3));
    }
}
