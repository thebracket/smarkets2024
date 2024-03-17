#/usr/bin/python3
import time
from librfib import recur_fibo, fibo_range

print("Single thread")
results = []
t0 = time.time()
for i in range(40):
    results.append(recur_fibo(i))
t1 = time.time()

print(results)
print("Time: ", t1-t0)

print("\n\nMulti thread")
t0 = time.time()
results = fibo_range(40)
t1 = time.time()
print(results)
print("Time: ", t1-t0)
