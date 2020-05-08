#include<iostream>
#include<vector>
#include <algorithm>
#include <tuple>
using namespace std;


tuple<int, long> get_max_idx(vector<long> &A, vector<long> &B, vector<long> &C,
                int x_i, int y_i, int z_i){
    int max_idx = 0, max_i = x_i;
    long min_num = A[x_i], res_sum;
    if(y_i > 0 && (max_i < 1 | min_num > B[y_i])){
        max_idx = 1;
        max_i = y_i;
        min_num = B[y_i];
    }
    if(z_i > 0 && (max_i < 1 | min_num > C[z_i])){
        max_idx = 2;
        max_i = z_i;
        min_num = C[z_i];
    }
    if(max_idx == 0)
        x_i--;
    else if(max_idx == 1)
        y_i--;
    else if(max_idx == 2)
        z_i--;
    else
        cout << "ERROR: max_idx is " << max_idx << endl;
    res_sum = A[x_i] + B[y_i] + C[z_i];
    tuple<int, long> res = make_tuple(max_idx, res_sum);
    return res;
}


int main(){
    /* greedy に探索すればよろし */
    // def inputs
    int X, Y, Z, K;
    // parse inputs
    cin >> X >> Y >> Z >> K;
    int x_i = X - 1, y_i = Y - 1, z_i = Z - 1;
    vector<long> reses(K);
    vector<long> A(X), B(Y), C(Z);
    for(int i = 0; i < X; ++i)
        cin >> A[i];
    for(int i = 0; i < Y; ++i)
        cin >> B[i];
    for(int i = 0; i < Z; ++i)
        cin >> C[i];
    sort(A.begin(), A.end());
    sort(B.begin(), B.end());
    sort(C.begin(), C.end());

    // calc reses
    for(int i = 1; i < K; ++i){
        tuple<int, long> res = get_max_idx(A, B, C, x_i, y_i, z_i);
        int max_idx = get<0>(res);
        reses[i] = get<1>(res);
        cout << "res_idx : " << max_idx << endl;
        if(max_idx == 0)
            x_i--;
        else if(max_idx == 1)
            y_i--;
        else if(max_idx == 2)
            z_i--;
        else
            cout << "ERROR" << endl;
        cout << x_i << " : " << y_i << " : " << z_i << endl;
    }

    // output
    for(int i = 0; i < K; ++i)
        cout << reses[i] << endl;

    return 0;
}
