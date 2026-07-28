#include <iostream>
#include <string>
#include <vector>

using namespace std;
// dungeons // 던전 배열 
// visited // 지금 이 경로에서 해당 던전을 다녀왔는지 
// k : 피로도 
// count: 지금 이 경로(ex. 1-3-4-2) 하나에서 몇 개를 탐험했는지
// answer: 지금까지 시도해본 모든 경로들 중에서, 가장 컸던 count가 뭐였는지 

// 가능한 모든 순서(순열)를 다 시도
void dfs(vector<vector<int>>& dungeons, vector<bool>& visited, int k, int count, int&answer) {
    answer = max(answer, count); // 지금 이 경로에서 여기까지 count개를 탐험했다는 사실 자체는 이미 확정된 값
                                 // 함수에 들어오자마자 일단 그 사실부터 기록 
    
    for(int i = 0; i < dungeons.size(); i++) {
        if(visited[i]) { // 방문한 적이 있으면 건너뜀
            continue;
        } else if(k >= dungeons[i][0]) { // 방문한 적이 없고, 조건 만족하면 
            visited[i] = true;  
            
            dfs(dungeons, visited, k-dungeons[i][1], count+1, answer); // k, count 자체를 건드리지 말고, 호출하는 자리에서 바로 계산해서 넘기기 
            
            visited[i] = false; // 원상 복구 필요 
        }
    }
}

int solution(int k, vector<vector<int>> dungeons) {
    int answer = -1;
    vector<bool> visited(dungeons.size(), false);
    int count = 0; 
    
    dfs(dungeons, visited, k, count, answer); // 재귀호출 
    
    return answer;
}

int main() {
    cout << solution(80, {{80,20}, {50,40}, {30,10}}) << endl;
}