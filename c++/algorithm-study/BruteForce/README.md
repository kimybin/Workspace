### 2. MockExam

#### 🌟 접근 방식
**1. 반복 패턴 인덱싱: 나머지 연산(%)**
- 각 수포자의 찍기 패턴은 정해진 길이(5, 8, 10)만큼 반복된다.
- i번째 문제에서 패턴의 몇 번째 값을 봐야 하는가?
    - 정답: 패턴배열[i % 패턴배열.size()]
    - 예: `num1[i % num1.size()]`
    - `i=0→0, i=1→1, ..., i=4→4, i=5→0 (다시 반복!)`

**2.  std::max()로 여러 값 비교하기**
- `int maxScore = max({correct1, correct2, correct3}); // 초기화 리스트로 3개 이상 비교 가능`
- `max(a, b)`는 인자 2개만 받지만, `max({a, b, c})`처럼 중괄호로 묶으면 여러 개 비교 가능
- `int result = max(max(a, b), c);` 이런 식으로도 사용 가능

#### 🚀 트러블 슈팅 
```
for(int i = 0; i < num1.size(); i++)          // 패턴 길이만큼만 돎 → 문제 다 못 봄
for(int i = 0; i < i%num1.size(); i++)         // i=0일 때부터 조건이 이상함
for(int i = 0; num1.size()%i < answers.size(); i++) // i=0일 때 0으로 나눔 (division by zero!)
```

- 반복문을 몇 번 돌지 `answers.size() (문제 총 개수)`
- 패턴 배열의 몇 번째를 볼지 `i % 패턴배열.size()`

<br>

**⭕️ 참고용 좋은 코드**
```
vector<int> one = {1,2,3,4,5};
vector<int> two = {2,1,2,3,2,4,2,5};
vector<int> thr = {3,3,1,1,2,2,4,4,5,5};

vector<int> solution(vector<int> answers) {
    vector<int> answer;
    vector<int> they(3);
    for(int i=0; i<answers.size(); i++) {
        if(answers[i] == one[i%one.size()]) they[0]++;
        if(answers[i] == two[i%two.size()]) they[1]++;
        if(answers[i] == thr[i%thr.size()]) they[2]++;
    }
    int they_max = *max_element(they.begin(),they.end());
    for(int i = 0; i< 3; i++) {
        if(they[i] == they_max) answer.push_back(i+1);
    }
    return answer;
}
```

---

### 3. Find_Prime_Num
#### 문제 | 숫자 문자열의 조각들을 조합해 만들 수 있는 소수의 개수 구하기 

#### 🌟 접근 방식
**1. 백트래킹으로 만들 수 있는 모든 숫자 구하기**
- 각 자리(인덱스)를 방문 체크(visited)하면서 순서대로 뽑아 picked에 이어붙임
- 재귀 depth가 깊어질수록 picked의 길이가 늘어나는데, 이 과정 자체가 "몇 개를 뽑을지"와 "어떤 순서로 배치할지"를 동시에 해결함 (별도 로직 불필요)
- picked가 비어있지 않을 때마다(=1글자 이상 뽑혔을 때마다) 그 순간의 값을 결과에 저장
- 재귀 호출 후 visited[i] = false, picked.pop_back()으로 원상복구 (백트래킹)

**2. 중복 제거**
- 같은 숫자가 여러 경로로 만들어질 수 있음 (예: "11"이 인덱스 0,1 / 1,0 두 경로로 나옴)
- set<int>에 저장하면 중복이 자동 제거됨
- 문자열 상태로 저장하면 "01"과 "1"이 다르게 취급되므로, stoi()로 정수 변환 후 저장

**3. 소수 판별**
- 약수는 항상 쌍으로 나오고, 그중 하나는 반드시 √n 이하이므로 2부터 √n까지만 확인하면 충분
- i * i <= n으로 실수 오차 없이 정수 비교

#### ⚙️ 함수 구조
```
bool isPrime(int n) { ... }          // 소수 판별 (독립 함수)
void dfs(numbers, picked, visited, s) { ... }  // 만들 수 있는 모든 숫자를 set에 저장
int solution(string numbers) {
    // 1. visited, set 초기화
    // 2. dfs 호출 → set에 결과 채워짐
    // 3. set 순회하며 소수 개수 세기
}
```

#### 🚀 트러블 슈팅
- 참조(&) 매개변수: visited, set처럼 재귀 전체에서 값이 유지돼야 하는 건 반드시 참조로 넘겨야 함. 참조 없이 넘기면 복사본이라 함수 밖에 영향 없음
- 재귀 호출의 역할: for문(i++)은 "같은 depth 안에서 다음 후보 시도", 재귀 호출은 "다음 depth(다음 자리)로 진행" — 이 둘의 역할이 다름을 헷갈렸었음
- 함수 선언 vs 호출: 선언부에는 타입 명시(set<int>& s), 호출부에는 변수 이름만(s)
- set은 인덱스 접근 불가: for(int i=0; i<s.size(); i++)가 아니라 for(int x : s)로 순회해야 함

---

### 4. 피로도 
#### 문제 | 현재 피로도 k와 던전별 [최소 필요 피로도, 소모 피로도] 목록이 주어질 때, 하루 동안 탐험할 수 있는 최대 던전 수를 구하는 문제. (던전 개수 ≤ 8)
- 같은 던전 집합이라도 방문 순서에 따라 탐험 가능한 개수가 달라짐이 핵심.
- 던전 개수가 최대 8개라 8! = 40,320가지 → 완전탐색(DFS)으로 모든 순서를 다 시도해도 충분히 빠름.

#### 🌟 접근 방식
1. visited 배열로 이미 탐험한 던전을 표시하면서, 매 단계마다 아직 안 간 던전 중 갈 수 있는 곳을 전부 시도하는 DFS + 백트래킹.

2. 재귀 함수가 호출될 때마다 "지금 이 경로에서 몇 개를 탐험했는지"(count)를 answer와 비교해서 최댓값을 갱신.

#### ⚙️ 함수 구조
```cpp
void dfs(vector<vector<int>>& dungeons, vector<bool>& visited, int k, int count, int& answer) {
    answer = max(answer, count);

    for (int i = 0; i < dungeons.size(); i++) {
        if (visited[i]) continue;
        if (k >= dungeons[i][0]) {
            visited[i] = true;
            dfs(dungeons, visited, k - dungeons[i][1], count + 1, answer);
            visited[i] = false;
        }
    }
}

int solution(int k, vector<vector<int>> dungeons) {
    int answer = 0;
    vector<bool> visited(dungeons.size(), false);
    dfs(dungeons, visited, k, 0, answer);
    return answer;
}
```


#### 🚀 트러블 슈팅
1. 참조(reference) vs 값(value) 전달 k, count처럼 "이 가지에서만 유효해야 하는 값"은 값으로 넘겨서 형제 가지끼리 서로 영향을 안 주게 함. 반대로 answer처럼 "전체 탐색에서 계속 누적되어야 하는 값"은 참조로 넘겨서 모든 가지가 공유하도록 함.
2. visited 원상복구 재귀 호출이 끝나고 형제 가지로 넘어갈 때는 visited[i] = false로 되돌려야, 그 형제 가지가 부모의 원래 상태에서 다시 시작할 수 있음. (참조로 넘긴 값은 자동으로 안 돌아오므로 수동 복구 필요)
3. answer 갱신 위치 for문 안에 넣으면 continue에 걸려 리프(더 못 가는 지점)에서 실행이 안 되는 버그 발생. 함수 맨 앞에 두면 리프든 아니든 매 호출마다 무조건 한 번 실행되어 놓치는 경우가 없음.
4. answer 초기값 count는 항상 0 이상이고, dfs가 count=0인 상태로 최소 한 번은 호출되므로 초기값을 -1로 하든 0으로 하든 결과는 같음. 핵심 조건은 "실제 정답보다 크지 않은 값"이면 됨.


#### ⭕️ 시간복잡도
- 최악의 경우 O(N!) (N ≤ 8이라 완전탐색으로 충분)

#### 🧐 다음에 볼 것
- DFS/백트래킹 기본기가 약하다고 느껴서 별도 연습 문제 세트로 재귀 → visited 관리 → 값/참조 구분 → 가지치기 순서로 복습 예정.