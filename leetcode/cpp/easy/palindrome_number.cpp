#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

class Solution {
   public:
    bool isPalindrome(int x) {
        string num_string = to_string(x);
        string reversed = num_string;
        reverse(reversed.begin(), reversed.end());
        return num_string == reversed;
    }
};