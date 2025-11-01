#include <iostream>
#include <vector>
using namespace std;

int main() {
    // Cases(t)
    int t;
    cin >> t;
    while (t--) {
        // Problems
        int n;
        cin >> n;
        vector<int> V(n, {});
        int w = 0;
        // [a1, a2, a3...an]
        for (int i = 0; i < n; i++)
            cin >> V[i];
        int index = 0;
        for (int i = 0; i < n; i++) {
            int c;
            cin >> c;
            (c < V[index]) ? w++ : index++;
        }
        cout << w << "\n";
    }
    return 0;
}