import sys
import time

def reverse_array(arr):
    start = 0
    end = len(arr) - 1
    while start < end:
        arr[start], arr[end] = arr[end], arr[start]
        start += 1
        end -= 1

size = int(sys.argv[1])

start_time = time.time_ns()
print(f"Start time: {start_time} ns")

arr = list(range(size))
reverse_array(arr)

end_time = time.time_ns()
execution_time = (end_time - start_time) / 1e9
print(f"Execution time: {execution_time:.9f} seconds")
print(f"Reversed array of size {size}")
