// Using catch-all error types like `Box<dyn Error>` isn't recommended for
// library code where callers might want to make decisions based on the error
// content instead of printing it out or propagating it further. Here, we define
// a custom error type to make it possible for callers to decide what to do next
// when our function returns an error.

use std::num::ParseIntError;

// 값 생성 시 발생할 수 있는 에러 종류
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative, // 음수인 경우
    Zero,     // 0인 경우
}

// parse() 함수에서 반환할 통합 에러 타입
// Box<dyn Error> 대신 이걸 쓰면 호출자가 에러 종류를 구분해서 처리할 수 있음
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError), // 값 생성 에러 (음수, 0)
    ParseInt(ParseIntError), // 문자열 → 숫자 변환 에러 ("not a number" 같은 경우)
}

impl ParsePosNonzeroError {
    // CreationError → ParsePosNonzeroError 로 변환
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // ParseIntError → ParsePosNonzeroError 로 변환
    fn from_parse_int(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // s.parse() 는 표준 라이브러리의 문자열→숫자 변환 함수
        // 실패 시 ParseIntError 반환 → map_err로 ParsePosNonzeroError 타입으로 변환
        // ? 는 에러면 조기 반환, 성공이면 x에 값 바인딩
        let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;
        // CreationError 도 같은 방식으로 ParsePosNonzeroError 로 변환 (? 없어도 마지막 값이 반환됨)
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
