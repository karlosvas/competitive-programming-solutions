#include <iostream>
#include <vector>
using namespace std;

int main() {
    int t;
    cin >> t;
    while (t--) {
        int n, m;
        cin >> n >> m;
        if (n >= m && (m % 2 == n % 2))
            cout << "Yes\n";
        else
            cout << "No\n";
    }
    return 0;
}