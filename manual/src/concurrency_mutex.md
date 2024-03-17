# Fearless Mutexes

You have have run into Mutexes in Python:

> This is available in `code/mutex.py`

```python
#!/usr/bin/python3
from threading import Thread, Lock

counter = 0
mutex = Lock()

def one():
    return 1

def adder():
    global counter
    for _ in range(1000000):
        with mutex:
            counter = counter + one()

threads = []
for i in range(10):
    thread = Thread(target=adder)
    thread.start()
    threads.append(thread)

for thread in threads:
    thread.join()

print(counter)
```

We declare "mutex" to be a `Lock()` and then the `with mutex` locks it. This prevents your race condition.

