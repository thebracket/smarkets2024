#!/usr/bin/python3
import threading

counter = 0

def one():
    return 1

def adder():
    global counter
    for _ in range(1000000):
        counter = counter + one()

threads = []
for i in range(10):
    thread = threading.Thread(target=adder)
    thread.start()
    threads.append(thread)

for thread in threads:
    thread.join()

print(counter)