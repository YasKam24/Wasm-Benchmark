import sys
import time

def linear_search(arr, target):
    for i, val in enumerate(arr):
        if val == target:
            return i
    return -1

size = int(sys.argv[1])

start_time = time.time_ns()
print(f"Start time: {start_time} ns")

arr = list(range(size))
target = size // 2
result = linear_search(arr, target)

end_time = time.time_ns()
execution_time = (end_time - start_time) / 1e9
print(f"Execution time: {execution_time:.9f} seconds")
print(f"Found target {target} at index {result} in array of size {size}")
