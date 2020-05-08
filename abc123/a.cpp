#include<iostream>
#include<string>
#include<vector>
#include<sstream>
using namespace std;

int main(){
    // def inputs
    int num_in, k, i = 0;
    vector<int> num_ins;
    string exists = ":(";
    string not_exist = "Yay!";

    // parse input
    while(cin >> num_in){
        if(i == 5){
            k = num_in;
            break;
        }else{
            num_ins.push_back(num_in);
            i++;
        }
    }

    // brute force
    for(int p_i = 0; p_i < 5; ++p_i){
        for(int q_i = 5; p_i < q_i; --q_i){
            if(num_ins[q_i] - num_ins[p_i] > k){
                cout << exists << endl;
                return 0;
            }
        }
    }
    cout << not_exist << endl;

    return 0;
}
