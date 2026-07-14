#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;
    
vector<int> solution(vector<int> answers) {
    vector<int> answer;
    
    vector<int> num1 = {1, 2, 3, 4, 5}; 
    vector<int> num2 = {2, 1, 2, 3, 2, 4, 2, 5}; 
    vector<int> num3 = {3, 3, 1, 1, 2, 2, 4, 4, 5, 5}; 
    int correct1 = 0, correct2 = 0, correct3 = 0; // correct number counting

    for(int i = 0; i < answers.size(); i++) {
        if(num1[i%num1.size()] == answers[i]) 
            correct1++;
    }
    for(int i = 0; i < answers.size(); i++) {
        if(num2[i%num2.size()] == answers[i]) 
            correct2++;
    }
    for(int i = 0; i < answers.size(); i++) {
        if(num3[i%num3.size()] == answers[i]) 
            correct3++;
    }
    
    // Evaluate if statements 1, 2, and 3 independently in order and execute push_back. 
    // If there are multiple tied results, sort them in ascending order.
    int maxScore = max({correct1, correct2, correct3});
    if(maxScore == correct1) answer.push_back(1);
    if(maxScore == correct2) answer.push_back(2);
    if(maxScore == correct3) answer.push_back(3);
    
    return answer;
}

int main() {
    // Test code
    for(int x : solution({1,2,3,4,5})) cout << x << " " << endl;
    for(int x: solution({1,3,2,4,2})) cout << x << " ";

    return 0; 
}