import sys
import time
from datetime import datetime




def factorial(n):
    if n == 0:
        return 1
    return n * factorial(n-1)

n = int(sys.argv[1])

start_time = time.time_ns()
print(f"Start time: {start_time} ns")

print(factorial(n))

end_time = time.time_ns()
execution_time = (end_time - start_time) / 1e9
print(f"Execution time: {execution_time:.9f} seconds")
