#include <iostream>
#include <string>
#include <vector>
#include <set>

using namespace std;

// 완전 탐색 
void dfs(string numbers, string picked, vector<bool>& visited, set<int>& s) {
    if(!picked.empty()) { // 1글자 이상 뽑혔을 경우 저장 
        s.insert(stoi(picked));
    }
    
    for(int i = 0; i < numbers.size(); i++) {
        if(visited[i]) {  // 방문한 경험 체크 
            continue;
        } else {
            picked += numbers[i];
            visited[i] = true;
            
            dfs(numbers, picked, visited, s); // 재귀 호출
            
            visited[i] = false; 
            picked.pop_back();
        }
    }
}

// 소수 판별 
bool isPrime(int n) {
    if (n < 2) return false;
        for (int i = 2; i * i <= n; i++) {
        if (n % i == 0) return false;
    }
    return true;
}

int solution(string numbers) {
    int answers = 0; 
    vector<bool> visited(numbers.size(), false); 
    set<int> s; 
        
    dfs(numbers, "", visited, s);
    
    for(int x : s) {
        if(isPrime(x)) answers++; 
    }    
    return answers;
}

int main() {
    cout << solution("17") << endl;   // 3
    cout << solution("011") << endl;  // 2

    return 0;
}