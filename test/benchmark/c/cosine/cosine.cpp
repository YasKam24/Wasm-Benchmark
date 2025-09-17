#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <chrono>
#include <inttypes.h> // for PRId64
using namespace std;

int main(int argc, char* argv[]) {
    auto start_time = chrono::system_clock::now();
    int64_t ns = chrono::duration_cast<chrono::nanoseconds>(start_time.time_since_epoch()).count();

    if (argc < 2) {
        printf("Usage: %s <angle_in_radians>\n", argv[0]);
        return 1;
    }

    double angle = atof(argv[1]);
    double result = cos(angle);

    printf("Start time: %" PRId64 " ns\n", ns);
    printf("Cosine of %f radians is: %f\n", angle, result);

    auto end_time = chrono::system_clock::now();
    int64_t duration_ns = chrono::duration_cast<chrono::nanoseconds>(end_time - start_time).count();
    double duration_s = duration_ns / 1e9;

    printf("Execution time: %f seconds\n", duration_s);
    return 0;
}
