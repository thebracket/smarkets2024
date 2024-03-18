# Arenas

Another way to solve cyclic references (and avoid memory fragmentation) is to use an arena. That is: you preallocate a blob of memory, and allocate within it. Then you can drop the whole thing at once, with no risk of losing anything. You really only need this for extreme cases.

See the `code/arena_demo` project.

> This is a very brief visit to an advanced topic. You probably don't need it, but you need to know that its available.

## SlotMaps

Another good way to handle large amounts of data referring to one another is to add a layer of indirection, and store the data inside a "slot map". You can then dispose of some or all of it at once. It's similar to storing everything in a `HashMap` (a Dictionary), but can be faster. In particular, it is better at *reclaiming* memory without having to move things around - so references remain valid. So long as you allocate enough memory up-front, insertion, deletion and indexing are all `O(1)`---very little added complexity.

