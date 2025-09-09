import sys
import time
from datetime import datetime

def factors(n):
    result = []
    for i in range(1, n + 1):
        if n % i == 0:
            result.append(i)
    return result

n = int(sys.argv[1])
start_time = time.time_ns()
print(f"Start time: {start_time} ns")
print(factors(n))
end_time = time.time_ns()
execution_time = (end_time - start_time) / 1e9
print(f"Execution time: {execution_time:.9f} seconds")
