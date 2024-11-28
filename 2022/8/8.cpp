#include <bits/stdc++.h>
#include <fstream>
#include <string>
#include <vector>
using namespace std;

bool isVisible(const vector<string>& v, const int a, const int b) {
    for (int i = a + 1; i <) }

int main() {
    ifstream reader("input.txt");
    vector<string> v;
    string str;
    int n = 0;
    while (getline(reader, str)) { v.push_back(str); }
    for (int i = 1; i < v[0].size(); i++) {
        for (int j = 1; i < v.size(); j++) {
            if (v[i][j] >= v[i - 1][j]) }
    }
}
