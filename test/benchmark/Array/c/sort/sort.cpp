#include <stdio.h>
#include <stdlib.h>
#include <chrono>
#include <inttypes.h>
#include <algorithm>
using namespace std;

void bubble_sort(int arr[], int n) {
    for (int i = 0; i < n - 1; i++) {
        for (int j = 0; j < n - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

int main(int argc, char* argv[]) {
    auto start_time = chrono::system_clock::now();
    int64_t ns = chrono::duration_cast<chrono::nanoseconds>(start_time.time_since_epoch()).count();

    printf("Start time: %" PRId64 " ns\n", ns);

    int size = atoi(argv[1]);
    int* arr = (int*)malloc(size * sizeof(int));
    
    // Initialize array with random values
    for (int i = 0; i < size; i++) {
        arr[i] = rand() % 1000;
    }

    bubble_sort(arr, size);

    auto end_time = chrono::system_clock::now();
    int64_t duration_ns = chrono::duration_cast<chrono::nanoseconds>(end_time - start_time).count();
    double duration_s = duration_ns / 1e9;

    printf("Execution time: %f seconds\n", duration_s);
    printf("Sorted array of size %d\n", size);
    
    free(arr);
    return 0;
}
