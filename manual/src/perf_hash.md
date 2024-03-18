# Hashing

It's always really tempted to fall back on the `HashMap` (and set, etc.) when indexing things. This is especially true if you are used to Python dictionaries, which are giant hashmaps for everything! HashMaps are great for fast retrieval, but they are (relatively) slow for insertion.

```rust
use std::collections::HashSet;

fn main() {
    let mut v = Vec::with_capacity(1_000_000);
    let now = std::time::Instant::now();
    for i in 0 .. 1_000_000 {
        v.push(i);
    }
    println!("Inserted into vector in {} seconds", now.elapsed().as_secs_f32());

    let now = std::time::Instant::now();
    let mut s = HashSet::with_capacity(1_000_000);
    for i in 0 .. 1_000_000 {
        s.insert(i);
    }
    println!("Inserted into set in {} seconds", now.elapsed().as_secs_f32());

    use ahash::HashSetExt;
    let now = std::time::Instant::now();
    let mut s2 = ahash::HashSet::with_capacity(1_000_000);
    for i in 0 .. 1_000_000 {
        s2.insert(i);
    }
    println!("Inserted into ahash set in {} seconds", now.elapsed().as_secs_f32());

    let now = std::time::Instant::now();
    if let Some(n) = v.iter().find(|n| **n==700) {
        println!("Found item in vector in {}", now.elapsed().as_secs_f32());
    }

    let now = std::time::Instant::now();
    if let Some(n) = s.get(&700) {
        println!("Found item in set in {}", now.elapsed().as_secs_f32());
    }

    let now = std::time::Instant::now();
    if let Some(n) = s2.get(&700) {
        println!("Found item in ahash set in {}", now.elapsed().as_secs_f32());
    }
}
```

Here we:
* Insert into a vector
* Insert into a HashSet
* Use the `ahash` crate for a faster hashing function

Then we:
* Search the vector (fast)
* Search the set (really fast)
* Search the ahash set (ridiculously fast)

Even with the faster function, hash insertion is still quite a bit slower.

> Understand what your data's purpose is. If you are searching a lot, sets and maps are great. If you are primarily adding---vectors are much better.

## A Common Example

Say you have a list of 100,000 items and just want to know how many items are equal to each value. The natural way to do it is to iterate, build a `HashSet` of counters, and print the result. It works---but if its a critical item, it's far from the fastest way.


```rust
use rand::Rng;
use std::time::Instant;
use ahash::HashMap;
use ahash::HashMapExt;

const NUM_CITIES: usize = 100_000;

fn build_data() -> Vec<String> {
    const CITIES: [&str; 5] = [
        "London",
        "Paris",
        "Frankfurt",
        "New York",
        "Tokyo",
    ];

    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(NUM_CITIES);
    for _ in 0 .. NUM_CITIES {
        v.push(CITIES[rng.gen_range(0..5)].to_string())
    }
    v
}

fn map_count(cities: &[String]) {
    let now = Instant::now();
    let mut counter: HashMap<String, usize> = HashMap::new();
    for city in cities.iter() {
        let mut entry = counter.entry(city.to_string()).or_insert(0);
        *entry += 1;
    }
    println!("Counted cities (map) in {} seconds", now.elapsed().as_secs_f32());
    println!("{counter:?}");
}

fn iter_count(cities: &[String]) {
    use itertools::Itertools;

    let now = Instant::now();
    let mut cities: Vec<&String> = cities.iter().collect();
    cities.sort_unstable();
    let result: Vec<(usize, &String)> = cities.into_iter().dedup_with_count().collect();    
    println!("Counted cities (dedup) in {} seconds", now.elapsed().as_secs_f32());
    println!("{result:?}");
}

fn main() {
    let cities = build_data();
    map_count(&cities);
    iter_count(&cities);
}
```

So here we try two algorithms:

1. The first makes a HashMap. We've been clever and used `ahash` (which uses hardware acceleration for hash generation if available). We iterate, and count the items for each city. It completes in 0.040604386 seconds.
2. The second makes a vector of *references* to the original list (so as to not change it). The list is sorted, and we use a crate called `Itertools` to de-duplicate with counts. It completes in 0.023555184 seconds.

So using the non-hashy algorithm is twice as fast! This is especially pronounced when you have a *lot* of data.