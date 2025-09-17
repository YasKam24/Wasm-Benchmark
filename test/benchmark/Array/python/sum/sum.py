import sys
import time

def array_sum(arr):
    return sum(arr)

size = int(sys.argv[1])

start_time = time.time_ns()
print(f"Start time: {start_time} ns")

arr = list(range(1, size + 1))
sum_result = array_sum(arr)

end_time = time.time_ns()
execution_time = (end_time - start_time) / 1e9
print(f"Execution time: {execution_time:.9f} seconds")
print(f"Sum of array of size {size} is: {sum_result}")
