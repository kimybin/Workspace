use core::fmt;
use std::collections::HashMap;
use std::fs::{File}; 
use std::io::{BufRead, BufReader};
use std::println;
use clap::Parser;

#[allow(dead_code)] // 이 필드들 지금 안 써도 컴파일러야 경고하지 말거라 
struct LogEntry {
    timestamp: String,
    level: String, 
    message: String,
}

// 에러 처리
#[derive(Debug)]
enum LogParserError {
    FileNotFound(String),
    ParseError(String),
}

impl fmt::Display for LogParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            LogParserError::FileNotFound(path) => write!(f, "파일을 찾을 수 없습니다: {}", path),
            LogParserError::ParseError(line) => write!(f, "파싱 실패: {}", line),
        }
    }
}

// CLI 인자 정의 
#[derive(Parser)] // `derive` may only be applied to `struct`s, `enum`s and `union`s
#[command(name="log-parser")]
#[command(about = "로그 파일을 읽어 레벨별로 집계하고 필터링합니다")]
struct Args {
    // 분석할 로그 파일 경로
    #[arg(default_value = "sample.log")]
    file: String, 

    // 특정 레벨만 필터링 (예: ERROR)
    #[arg(long)]
    level: Option<String>,

    // 메세제에서 키워드 검색
    #[arg(long)]
    grep: Option<String>,

}


fn parse_line(line: &str) -> Result<LogEntry, LogParserError> {
    let stripped = line.strip_prefix('[') // 맨 앞 '[' 제거
        .ok_or_else(|| LogParserError::ParseError(line.to_string()))?;

    let mut parts = stripped.splitn(2, "]["); // "][" 기준으로 한 번만 분리

    let timestamp = parts.next()
        .ok_or_else(|| LogParserError::ParseError(line.to_string()))?
        .to_string(); // 이건 왜 to_string()를 한 번 더 하지? 

    let rest = parts.next()
        .ok_or_else(|| LogParserError::ParseError(line.to_string()))?; // "INFO] Server started..."

    // ================

    let mut parts2 = rest.splitn(2, "] "); // "] " 기준으로 한 번만 분리

    let level = parts2.next()
        .ok_or_else(|| LogParserError::ParseError(line.to_string()))?
        .to_string(); // 이건 왜 to_string()를 한 번 더 하지? 

    let message = parts2.next()
        .ok_or_else(|| LogParserError::ParseError(line.to_string()))?
        .to_string(); // 이건 왜 to_string()를 한 번 더 하지? 

    Ok(LogEntry { timestamp, level, message })

}

fn run(args: &Args) -> Result<(), LogParserError> {
    let path = "sample.log";
    let file = File::open(path).map_err(|_| LogParserError::FileNotFound(path.to_string()))?;
    let reader = BufReader::new(file);

    let mut counts: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line.map_err(|_| LogParserError::ParseError("줄 읽기 실패".to_string()))?;

        match parse_line(&line) {
            Ok(entry) => {
                // level 필터: 지정했는데 안 맞으면 건너뜀 
                if let Some(target_level) = &args.level {
                    if &entry.level != target_level{
                        continue;
                    }
                }

                // grep 필터: 지정했는데 메세지에 없으면 건너뜀
                if let Some(keyword) = &args.grep {
                    if !entry.message.contains(keyword.as_str()) {
                        continue;
                    }
                }


                println!("[{}] ({}) {}", entry.timestamp, entry.level, entry.message); 
                *counts.entry(entry.level.clone()).or_insert(0) += 1; 
            }
            Err(e) => {
                eprint!("경고: {}", e);
            }
        }
    }

    println!("=== 로그 레벨별 집계 ===");
    for(level, count) in &counts {
        println!("{}: {}건", level, count);
    }

    Ok(()) // 성공 (담을 값이 없어서 빈 값)
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(&args) {
        eprintln!("에러 발생: {}", e);
        std::process::exit(1);
    }
}