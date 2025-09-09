#include <iostream>
#include <chrono>
using namespace std;

int main() {
    auto now = chrono::system_clock::now();
    auto ns = chrono::duration_cast<chrono::nanoseconds>(now.time_since_epoch()).count();

    cout << ns << endl;
    return 0;
}

