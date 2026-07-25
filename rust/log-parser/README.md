### 💎 log-parser

- 로그 파일을 읽어 레벨별로 집계하고, 원하는 조건으로 필터링해주는 Rust CLI 도구 
- Ubunto 환경에서, Rust  개념 중 소유권, `Option`/`Result`, 에러 처리, 이터레이터, `HashMap`, CLI 인자 파싱까지 직접 손으로 익히기 위해 제작

---

### ⚙️ 기술 스택

- **Language**: Rust
- **CLI 파싱**: clap (derive API)

---
### 🚀 실행 방법

- 빌드 & 실행

```bash
cargo run -- [OPTIONS] [FILE]
```

`FILE`을 생략하면 기본값으로 `sample.log`를 읽음

---

### ➡️ 사용 예시

```bash
# 전체 로그 출력 + 레벨별 집계
cargo run

# ERROR 레벨만 필터링
cargo run -- --level ERROR

# 메시지에 특정 키워드가 포함된 줄만 필터링
cargo run -- --grep Redis

# 레벨 + 키워드 동시 필터링
cargo run -- --level ERROR --grep Failed

# 다른 로그 파일 지정
cargo run -- other.log --level WARN
```

지원하는 로그 형식: 

```
[타임스탬프][레벨] 메시지
```

예:
```
[2026-08-03 09:12:01][INFO] Server started on port 8080
[2026-08-03 09:18:47][ERROR] Failed to connect to Redis
```
---
### 🛠️ 주요 기능

- 로그 파일을 한 줄씩 읽어 `timestamp` / `level` / `message`로 파싱
- 레벨별 건수 집계 (`HashMap<String, i32>`)
- `--level`로 특정 레벨만 필터링
- `--grep`으로 메시지 내 키워드 검색
- 파싱에 실패한 줄은 프로그램을 중단하지 않고 경고만 출력한 뒤 계속 진행
- 커스텀 에러 타입(`LogParserError`)으로 "파일 없음"과 "파싱 실패"를 구분해서 처리
---

### 📌 트러블 슈팅

`Option<LogEntry>` → `Result<LogEntry, LogParserError>`로 바꾼 이유

- 처음엔 파싱 실패를 `Option`(성공/실패만 구분)으로 처리했는데, "왜 실패했는지"까지 알 수 있는 게 나을 것 같아 커스텀 에러 enum을 만들어 `Result`로 바꿈
- 덕분에 `FileNotFound`와 `ParseError`를 구분해서 다른 메시지를 보여줄 수 있음

`&str`과 `String`을 구분해서 쓴 이유

- `splitn`으로 문자열을 쪼갠 조각들은 원본을 빌린 `&str`
- 이 조각들이 함수 안에서 다시 쪼개는 재료로만 쓰이면 `&str` 그대로 두고, `LogEntry` 구조체에 담겨 함수 밖으로 반환돼야 하는 값(`timestamp`, `level`, `message`)만 `.to_string()`으로 복사해 독립된 `String`으로 만듦
- 그렇지 않으면 함수가 끝나면서 원본이 사라져 참조가 무효해지는 문제(dangling reference)가 발생 

---

### 🕹️ 아직 안 해본 것 / 추후 개선 아이디어

- 단위 테스트 추가
- 타임스탬프 기준 필터링
- 여러 파일 동시 처리