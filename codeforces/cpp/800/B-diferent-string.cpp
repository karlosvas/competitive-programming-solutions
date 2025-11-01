#include <algorithm>
#include <iostream>
using namespace std;

int main() {
    srand(time(0));
    int t;
    cin >> t;
    while (t--) {
        string s, r;
        cin >> s;
        r = s;
        char actual_string = s[0];
        bool pos = false;
        for (int i = 0; i < s.size(); i++) {
            if (s[i] != actual_string) {
                pos = true;
                break;
            }
            actual_string = s[i];
        }
        if (!pos)
            cout << "NO\n";
        else {
            while (s == r)
                random_shuffle(r.begin(), r.end());
            cout << "YES\n"
                 << r << "\n";
        }
    }
    return 0;
}