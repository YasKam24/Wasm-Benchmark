#include <stdio.h>
#include <stdlib.h>
#include <chrono>
#include <inttypes.h>
using namespace std;

int linear_search(int arr[], int n, int target) {
    for (int i = 0; i < n; i++) {
        if (arr[i] == target) {
            return i;
        }
    }
    return -1;
}

int main(int argc, char* argv[]) {
    auto start_time = chrono::system_clock::now();
    int64_t ns = chrono::duration_cast<chrono::nanoseconds>(start_time.time_since_epoch()).count();

    printf("Start time: %" PRId64 " ns\n", ns);

    int size = atoi(argv[1]);
    int* arr = (int*)malloc(size * sizeof(int));
    
    // Initialize array with sequential values
    for (int i = 0; i < size; i++) {
        arr[i] = i;
    }

    int target = size / 2; // Search for middle element
    int result = linear_search(arr, size, target);

    auto end_time = chrono::system_clock::now();
    int64_t duration_ns = chrono::duration_cast<chrono::nanoseconds>(end_time - start_time).count();
    double duration_s = duration_ns / 1e9;

    printf("Execution time: %f seconds\n", duration_s);
    printf("Found target %d at index %d in array of size %d\n", target, result, size);
    
    free(arr);
    return 0;
}
