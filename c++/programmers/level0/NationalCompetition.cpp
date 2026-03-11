#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <numeric>

using namespace std;

int solution(vector<int> rank, vector<bool> attendance) {
    int answer = 0;
    
    vector<int> idx(rank.size());
    iota(idx.begin(), idx.end(), 0);
    
    sort(idx.begin(), idx.end(), [&](int a, int b) {
        return rank[a] < rank[b]; // 오름차순
    }); 
    
    vector<int> result;
    for(int i = 0; i < idx.size(); i++) {
        if(attendance[idx[i]])
            result.push_back(idx[i]);
        if(result.size() == 3)
            break;
    }
    
    answer = 10000 * result[0] + 100 * result[1] + result[2];
    
    return answer;
}

int main() {
    cout << "Enter the size of rank and attendance: ";
    int size;
    cin >> size;

    vector<int> rank(size);
    cout << "Enter the element of rank: ";
    for(int i = 0; i < size; i++) 
        cin >> rank[i];
    
    vector<bool> attendance(size); 
    cout << "Enter the element of attendance: ";
    for(int i = 0; i < size; i++) { // bool은 cin >> 으로 직접 입력 불가 
        int tmp; 
        cin >> tmp; // int로 받고 변환 
        attendance[i] = (tmp==1); // 1이면 true, 0이면 false
    }

    int sum = solution(rank, attendance);
    cout << "The result is " << sum << endl;

    return 0;
}