import sys
import time
import random

def bubble_sort(arr):
    n = len(arr)
    for i in range(n - 1):
        for j in range(n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]

size = int(sys.argv[1])

start_time = time.time_ns()
print(f"Start time: {start_time} ns")

arr = [random.randint(0, 999) for _ in range(size)]
bubble_sort(arr)

end_time = time.time_ns()
execution_time = (end_time - start_time) / 1e9
print(f"Execution time: {execution_time:.9f} seconds")
print(f"Sorted array of size {size}")
