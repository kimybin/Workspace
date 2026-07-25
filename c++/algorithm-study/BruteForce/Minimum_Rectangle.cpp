#include <iostream>
#include <string>
#include <vector>
#include <utility>
#include <algorithm>

using namespace std;

int solution(vector<vector<int>> sizes) {
    int answer = 0;
    int maxW = 0; 
    int maxH = 0; 
    
    // 1. Normalize all cards to [larger value, smaller value] form
    for(auto& size : sizes) {
        if(size[0] < size[1])
            swap(size[0], size[1]);
            
        // 2. Find the max of size[0] and the max of size[1]
        maxW = max(maxW, size[0]);
        maxH = max(maxH, size[1]);
    }
    
    answer = maxW * maxH;     
    return answer;
}

int main() {
    // Test code 
    cout << solution({{60, 50}, {30, 70}, {60, 30}, {80, 40}}) << endl; 
    cout << solution({{10, 7}, {12, 3}, {8, 15}, {5, 15}}) << endl; 
    cout << solution({{14, 4}, {19, 6}, {18, 7}, {7, 11}}) << endl; 

    return 0; 

}