### 2. MockExam

**✅ 반복 패턴 인덱싱: 나머지 연산(%)**
- 각 수포자의 찍기 패턴은 정해진 길이(5, 8, 10)만큼 반복된다.
- i번째 문제에서 패턴의 몇 번째 값을 봐야 하는가?
    - 정답: 패턴배열[i % 패턴배열.size()]
    - 예: `num1[i % num1.size()]`
    - `i=0→0, i=1→1, ..., i=4→4, i=5→0 (다시 반복!)`

**✅ std::max()로 여러 값 비교하기**
- `int maxScore = max({correct1, correct2, correct3}); // 초기화 리스트로 3개 이상 비교 가능`
- `max(a, b)`는 인자 2개만 받지만, `max({a, b, c})`처럼 중괄호로 묶으면 여러 개 비교 가능
- `int result = max(max(a, b), c);` 이런 식으로도 사용 가능

**❌ 가장 많이 해맨 부분**
```
for(int i = 0; i < num1.size(); i++)          // 패턴 길이만큼만 돎 → 문제 다 못 봄
for(int i = 0; i < i%num1.size(); i++)         // i=0일 때부터 조건이 이상함
for(int i = 0; num1.size()%i < answers.size(); i++) // i=0일 때 0으로 나눔 (division by zero!)
```

- 반복문을 몇 번 돌지 `answers.size() (문제 총 개수)`
- 패턴 배열의 몇 번째를 볼지 `i % 패턴배열.size()`

---

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

