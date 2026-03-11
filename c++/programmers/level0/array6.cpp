#include <iostream>
#include <string>
#include <vector>

using namespace std;

vector<int> solution(vector<int> arr) {
    vector<int> stk;
    
    for(int i = 0; i < arr.size(); i++) {
        if(stk.empty() || (stk.back()!=arr[i])) { 
            stk.push_back(arr[i]);
        } else if(!stk.empty() && (stk.back()==arr[i])) {
            stk.pop_back();
        }
    }
    
    if(stk.empty()) 
        stk.push_back(-1);
    
    return stk;
}

int main() {
    cout << "Enter the size of arr: "; 
    int len; 
    cin >> len; // size of arr

    vector<int> arr(len); // make arr 
    cout << "Enter the element of arr: "; 
    for(int i = 0; i < len; i++) { // Enter element of arr
        cin >> arr[i];
    }

    vector<int> result = solution(arr); 

    cout << "result[";
    for(int i = 0; i < result.size(); i++) { // Print result
        cout << result[i] << endl;
    }

    cout << "]: " << endl;
}