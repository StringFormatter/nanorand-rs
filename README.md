
# This is currently not cryptographically-secure. CSPRNGs are planned to be added in the future, but do not use this in it's current state for cryptographically-secure random number generation.

# nanorand

`nanorand` is a Rust crate, meant for fast, high-level, zero-dependency random number generation.

## Examples

### Independent State
```rs
use nanorand::{RNG, WyRand};

fn main() {
    let mut rng = WyRand::new();
    println!("Random 64-bit number: {}", rng.rand());
    println!("Random number in 0-100 range: {}", rng.rand_range(0, 100));
}
```

### Global State  
(Requires the `atomics` feature, and a platform that supports atomics)
```rs
use nanorand::{RNG, WyRand};

fn main() {
    println!("Random 64-bit number: {}", WyRand::rand_global());
    println!("Random number in 0-100 range: {}", WyRand::rand_global_range(0, 100));
}
```

## RNG Implementations

**RNG**|**nanorand type**|**Output Size**|**Cryptographically Secure**|**Supports Global State**|**Speed (Internal State)**|**Speed (Global State)**|**Notes**|**Original Implementation**
:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:
wyrand|[nanohash::WyRand](src/rand/wyrand.rs)|64 bits (`u64`)|🚫|✅|2.5 ns|12 ns||https://github.com/lemire/testingRNG/blob/master/source/wyrand.h
Pcg64|[nanohash::Pcg64](src/rand/pcg64.rs)|64 bits (`u64`)|🚫|🚫|7 ns|N/A||https://github.com/rkern/pcg64


## Entropy Sources

* Unix-like (Linux, Android, macOS, iOS, FreeBSD, OpenBSD) - first `/dev/urandom`, else `/dev/random`, else system time. (`#[forbid(unsafe_code)]`)
* Windows - `BCryptGenRandom` with system-preferred RNG. (`#[deny(unsafe_code)]`)

## Manually Seeding

`nanorand` by default has the `ctor` feature enabled, which will seed the global state using the default entropy source.  
If needed, you can seed it yourself:

```rs
fn main() {
    nanorand::seed_global(42);
}
```

## Feature Flags

* `std` (default) - Enables Rust `std` lib features, such as seeding from OS entropy sources.  
* `atomics` (default) - Enables an atomic global state, used for things like `RNG::rand_global`.
* `ctor` (default) - Initializes the atomic global state at program initialization.
* `wyrand` (default) - Enable the "wyrand" RNG.
* `pcg64` (default) - Enable the "Pcg64" RNG.

## License

The zlib/libpng License

Copyright (c) `2020` `aspen`

This software is provided 'as-is', without any express or implied warranty. In
no event will the authors be held liable for any damages arising from the use of
this software.

Permission is granted to anyone to use this software for any purpose, including
commercial applications, and to alter it and redistribute it freely, subject to
the following restrictions:

1.  The origin of this software must not be misrepresented; you must not claim
    that you wrote the original software. If you use this software in a product,
    an acknowledgment in the product documentation would be appreciated but is
    not required.

2.  Altered source versions must be plainly marked as such, and must not be
    misrepresented as being the original software.

3.  This notice may not be removed or altered from any source distribution.
