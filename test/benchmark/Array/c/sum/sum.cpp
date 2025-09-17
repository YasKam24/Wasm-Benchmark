#include <stdio.h>
#include <stdlib.h>
#include <chrono>
#include <inttypes.h>
using namespace std;

long long array_sum(int arr[], int n) {
    long long sum = 0;
    for (int i = 0; i < n; i++) {
        sum += arr[i];
    }
    return sum;
}

int main(int argc, char* argv[]) {
    auto start_time = chrono::system_clock::now();
    int64_t ns = chrono::duration_cast<chrono::nanoseconds>(start_time.time_since_epoch()).count();

    printf("Start time: %" PRId64 " ns\n", ns);

    int size = atoi(argv[1]);
    int* arr = (int*)malloc(size * sizeof(int));
    
    // Initialize array with sequential values
    for (int i = 0; i < size; i++) {
        arr[i] = i + 1;
    }

    long long sum = array_sum(arr, size);

    auto end_time = chrono::system_clock::now();
    int64_t duration_ns = chrono::duration_cast<chrono::nanoseconds>(end_time - start_time).count();
    double duration_s = duration_ns / 1e9;

    printf("Execution time: %f seconds\n", duration_s);
    printf("Sum of array of size %d is: %lld\n", size, sum);
    
    free(arr);
    return 0;
}
