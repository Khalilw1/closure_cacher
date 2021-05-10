# closure_cacher
This library offer caching for a user provided closure.
There are two types of cachers inside:
- **Cacher** which copies the user input and takes ownership of that copy
- **RefCacher** which references user input and thus is bound to that input but doesn't involve copy

# Usage
```rust
use closure_cacher::Cacher;

let mut cacher = Cacher::new(|x| x + 1)
println("{}", cacher.get(&4)); // outputs 5
```

```rust
use closure_cacher::RefCacher;
let four = 4;
let mut cacher = RefCacher::new(|x| x + 1)
println("{}", cacher.get(&four));
```

# Contributions
If you have any feature request or enhancement in mind. I would love to add it :)


# Disclaimer
This is not a very critical library i.e. it does one job in a very standard way. 

You could usually just write it again.

I used it as part of my pass through the rust book and decided to go ahead and share so any feedback is appreciated.