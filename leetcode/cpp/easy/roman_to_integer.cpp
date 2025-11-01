#include <iostream>
#include <map>
#include <unordered_map>
using namespace std;

class Solution {
   public:
    int romanToInt(string s) {
        unordered_map<char, int> m = {{'I', 1}, {'V', 5}, {'X', 10}, {'L', 50}, {'C', 100}, {'D', 500}, {'M', 1000}};

        int sum = 0;
        for (int i = 0; i < s.size(); i++) {
            int actual = m[s[i]];
            int next = i < s.size() ? m[s[i + 1]] : 0;
            if (next > actual) {
                sum += (next - actual);
                i++;
            } else
                sum += actual;
        }
        return sum;
    }
};