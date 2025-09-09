import sys
import math
import time
from datetime import datetime


def sine(n):
    result = math.sin(n)
    return result

n = float(sys.argv[1])

start_time = time.time_ns()
print(f"Start time: {start_time} ns")
print(sine(n))
end_time = time.time_ns()
execution_time = (end_time - start_time) / 1e9
print(f"Execution time: {execution_time:.9f} seconds")
