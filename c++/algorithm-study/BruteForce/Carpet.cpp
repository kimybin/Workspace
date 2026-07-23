#include <iostream>
#include <string>
#include <vector>

using namespace std;

vector<int> solution(int brown, int yellow) {
    vector<int> answer;
    
    int total = brown + yellow; // brown이 가로 yellow가 세로
    
    int w = 1, h = 1; 
    for(int i = 1; i <= total; i++) { // i가 h(세로), total/i가 w(가로)
        if(total % i == 0) {
            h = i; 
            w = total/i;
            
            if(2*(w+h)-4 == brown && w >= h) {
                answer.push_back(w);
                break;
            }
                
        }
    }
    answer.push_back(h);
    
    return answer;
}

int main() {
    cout << "[";
    for(int x : solution(8, 1)) cout << x << " ";
    cout << "]" << endl; 

    cout << "[";
    for(int x : solution(8, 1)) cout << x << " ";
    cout << "]" << endl; 

    cout << "[";
    for(int x : solution(24, 24)) cout << x << " ";
    cout << "]" << endl; 

    return 0;
}