#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <chrono>
#include <inttypes.h> // for PRId64
using namespace std;

double factorial(int n) {
    double result = 1;
    for (int i = 2; i <= n; ++i) {
        result *= i;
    }
    return result;
}

int main(int argc, char* argv[]) {
    auto start_time = chrono::system_clock::now();
    int64_t ns = chrono::duration_cast<chrono::nanoseconds>(start_time.time_since_epoch()).count();

    printf("Start time: %" PRId64 " ns\n", ns);

    int n = atoi(argv[1]);
    double sum = 0;

    for (int i = 1; i <= n; ++i) {
        sum += factorial(i);
    }

    auto end_time = chrono::system_clock::now();
    int64_t duration_ns = chrono::duration_cast<chrono::nanoseconds>(end_time - start_time).count();
    double duration_s = duration_ns / 1e9;

    printf("Execution time: %f seconds\n", duration_s);

    printf("Sum of factorials from 1 to %d is: %.0f\n", n, sum);
    return 0;
}
