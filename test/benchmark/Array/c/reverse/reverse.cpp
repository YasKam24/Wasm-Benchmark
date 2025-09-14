#include <stdio.h>
#include <stdlib.h>
#include <chrono>
#include <inttypes.h>
using namespace std;

void reverse_array(int arr[], int n) {
    int start = 0;
    int end = n - 1;
    while (start < end) {
        int temp = arr[start];
        arr[start] = arr[end];
        arr[end] = temp;
        start++;
        end--;
    }
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

    reverse_array(arr, size);

    auto end_time = chrono::system_clock::now();
    int64_t duration_ns = chrono::duration_cast<chrono::nanoseconds>(end_time - start_time).count();
    double duration_s = duration_ns / 1e9;

    printf("Execution time: %f seconds\n", duration_s);
    printf("Reversed array of size %d\n", size);
    
    free(arr);
    return 0;
}
