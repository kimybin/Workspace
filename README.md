# Study Log

Personal study repository covering algorithm practice (C++) and systems programming fundamentals (Rust). 
유형별로 정리한 코딩테스트 풀이 저장소와 rust 공부를 위한 rustlings 풀이 저장소입니다. 

## Structure

```
.
├── c++/
│   └── programmers/       # Programmers coding test solutions, by algorithm type
│       ├── BruteForce/
│       ├── Hash/
│       ├── Sort/
│       ├── StackQueue/
│       ├── Heap/
│       ├── BinarySearch/
│       ├── DFSBFS/
│       ├── Greedy/
│       ├── DP/
│       └── Graph/
├── rust/
│   └── easy_rust/         # Rust fundamentals: ownership, borrowing, Option/Result
└── rustlings/
    ├── exercises/          # Official rustlings exercises
    └── solutions/          # My solutions, numbered by topic (00_intro → 23_conversions)
```

## C++ — Algorithm Study

Solutions to Programmers coding test problems, organized by algorithm type.

**Progress**

| Category | Solved | Notes |
|---|---|---|
| Hash | 0 / 5 | |
| Stack/Queue | 0 / 6 | |
| Heap | 0 / 3 | |
| Sort | 0 / 3 | |
| Brute Force | 1 / 7 | |
| Greedy | 0 / 6 | |
| Dynamic Programming | 0 / 5 | |
| DFS/BFS | 0 / 7 | |
| Binary Search | 0 / 2 | |
| Graph | 0 / 3 | |

- Compiler (reference/judge environment): g++, C++17
- Each problem: `{Category}/{ProblemName}/{ProblemName}.cpp`

```bash
g++ -std=c++17 -o solution solution.cpp
./solution
```

## Rust — Fundamentals & Rustlings

Working through Rust fundamentals in preparation for a graduation project (Rust agent ↔ Java Spring Boot server via gRPC).

- `rust/easy_rust`: self-study exercises on ownership, borrowing, `&str` vs `String`, `Option`/`Result`, `HashMap`
- `rustlings/solutions`: solutions to the official [rustlings](https://github.com/rust-lang/rustlings) exercises, covering structs, enums, traits, lifetimes, iterators, smart pointers, threads, and more

```bash
cd rustlings
rustlings watch
```

## Notes

- Detailed problem-solving notes (approach, trade-offs, mistakes) are kept separately in Korean:
- [https://app.notion.com/p/Easy-Rust-2d2d1f03a723807889fcdb0cb6526514?source=copy_link]
- Commit convention: see [COMMIT_CONVENTION.md](./COMMIT_CONVENTION.md)
