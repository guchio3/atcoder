#include<iostream>
#include<vector>
using namespace std;


long get_min_element(vector<long>& in_vec){
    long min_element = in_vec[0];
    for(int i = 1; i < in_vec.size(); ++i)
        if(in_vec[i] < min_element)
            min_element = in_vec[i];
    return min_element;
}


int main(){
    // def inputs
    long res = 0, num_in, N, min_throughput, stuck_num;
    vector<long> cities(6);

    // parse inputs
    for(int i = 0; i < 6; ++i){
        cin >> num_in;
        cities[i] = num_in;
    }
    // the first element is N
    N = cities[0];
    min_throughput = get_min_element(cities);
    // get stuck_num
    stuck_num = long(N / min_throughput);
    if(N % min_throughput != 0)
        ++stuck_num;
    res = 5 + stuck_num - 1;

    cout << res << endl;

    return 0;
}
