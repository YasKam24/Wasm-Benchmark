#include <cstdint>
#include <cstdio>
#include <cstdlib>   // for std::atoi
#include <chrono>
#include <inttypes.h> // for PRId64
using namespace std;

int main(int argc, char* argv[]) {
    auto start_time = chrono::system_clock::now();
    int64_t ns = chrono::duration_cast<chrono::nanoseconds>(start_time.time_since_epoch()).count();

    printf("Start time: %" PRId64 " ns\n", ns);

    if (argc < 2) {
        printf("Usage: %s <number>\n", argv[0]);
        return 1;
    }

    int32_t n = std::atoi(argv[1]);  // atoi is in <cstdlib>
    int32_t count = 0;

    for (int32_t i = 1; i <= n; ++i) {
        if (n % i == 0) {
            printf("%d\n", i);
            count++;
        }
    }
    printf("Total factors: %d\n", count);

    auto end_time = chrono::system_clock::now();
    int64_t duration_ns = chrono::duration_cast<chrono::nanoseconds>(end_time - start_time).count();
    double duration_s = duration_ns / 1e9;

    printf("Execution time: %f seconds\n", duration_s);
    return 0;
}
