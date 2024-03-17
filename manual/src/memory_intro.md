# Rust Memory Management

> Rust **is not** a garbage collected language---but it can feel like one in normal usage. You very rarely have to resort to actual direct memory management; but you can if you need to!

Anyone here familiar with C and C++. Does this give you nightmares?

```cpp
#include <stdio.h>
#include <stdlib.h>

struct MyStruct {
    int number;
};

int main() {
    MyStruct * heap_struct = new MyStruct();
    // Forget this and you have a memory leak!
    delete heap_struct;
    
    struct MyStruct * c_struct = (struct MyStruct *)malloc(sizeof(struct MyStruct));
    // Same again
    free(c_struct);
    
    return 0;
}
```

When you call `new` or `free`, you are *allocating* memory on the heap (as opposed to the stack). If you forget to clean up after yourself, you have a memory leak. In this case, it's harmless---the program is going away anyway. In a big program, leaking from time to time can gradually increase your memory usage until the whole program is killed by the operating system.

Python, Java, Go, C# all solve this by being *garbage collected* languages. You allocate, the system notices that you aren't using a variable anymore---and it is destroyed for you. That's *wonderful*---but it can be a problem when you need very high performance. From time to time, the garbage collector will run one of various algorithms to find the unused space and clean it up. You have limited control over when it does this. So you can often have a system with really predictable latency *most of the time*, and the occasional sudden performance burp.

Rust doesn't do that.