#include <iostream>
#include <string>
#include <vector>

using namespace std;

vector<int> solution(vector<int> arr, vector<vector<int>> intervals) {

    vector<int> answer(arr.begin()+intervals[0][0], arr.begin()+intervals[0][1]+1);
    answer.insert(answer.end(), arr.begin()+intervals[1][0], arr.begin()+intervals[1][1]+1);
    
    return answer;
}

int main() {
    int n;
    cout << "arr 크기 입력: ";
    cin >> n;

    vector<int> arr(n); 
    cout << "arr 원소 입력 (" << n << "개): ";
    for (int i = 0; i < n; i++) 
        cin >> arr[i];

    vector<vector<int>> intervals(2, vector<int>(2));
    cout << "구간1 입력 (a1 b1): ";
    cin >> intervals[0][0] >> intervals[0][1];
    cout << "구간2 입력 (a2 b2): ";
    cin >> intervals[1][0] >> intervals[1][1];

    vector<int> result = solution(arr, intervals);

    cout << "result: [";
    for (int i = 0; i < result.size(); i++) {
        if (i > 0) cout << ", ";
        cout << result[i];
    }
    cout << "]" << endl;

    return 0;
}