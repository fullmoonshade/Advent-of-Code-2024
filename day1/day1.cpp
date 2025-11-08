#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <cmath>
using namespace std;

int main() {
    vector<int> leftList, rightList;
    int a, b;

    ifstream file("input.txt");
    if (!file) {
        cerr << "Error: Could not open input file" << endl;
        return 1;
    }

    while (file >> a >> b) {
        leftList.push_back(a);
        rightList.push_back(b);
    }

    sort(leftList.begin(), leftList.end());
    sort(rightList.begin(), rightList.end());

    long long totalDistance = 0;
    for (size_t i = 0; i < leftList.size(); i++) {
        totalDistance += abs(leftList[i] - rightList[i]);
    }

    cout << totalDistance << endl;
    return 0;
}
