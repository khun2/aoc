#include <bits/stdc++.h>
using namespace std;

bool isVisible(const vector<string>& v, const int a, const int b) {
    int blockedSides = 0;
    for (int i = a + 1; i < v.size(); i++) {
        if (v[a][b] <= v[i][b]) blockedSides++;
    }
    for (int i = a - 1; i >= 0; i--) {
        if (v[a][b] <= v[i][b]) blockedSides++;
    }
    for (int j = b + 1; j < v[0].size(); j++) {
        if (v[a][b] <= v[a][j]) blockedSides++;
    }
    for (int j = b - 1; j >= 0; j--) {
        if (v[a][b] <= v[a][j]) blockedSides++;
    }
    return blockedSides != 4;
}

int main() {
    ifstream reader("input.txt");
    vector<string> v;
    string str;
    int n = 0;
    while (getline(reader, str)) { v.push_back(str); }
    for (int i = 1; i < v.size() - 1; i++) {
        for (int j = 1; j < v[0].size() - 1; j++) {
            if (!isVisible(v, i, j)) {
                cerr << i + 1 << ' ' << j + 1 << " is not visible\n";
                n++;
            }
        }
    }
    cout << n;
}
