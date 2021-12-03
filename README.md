## ***`collectfiles`***

---

Collects accurate files while running in parallel through directories. (Simple, Fast, Powerful)


[![CI][ci-badge]][ci-url]
[![Crates.io][crates-badge]][crates-url]
[![Licensed][license-badge]][license-url]
[![Twitter][twitter-badge]][twitter-url]

[ci-badge]: https://github.com/just-do-halee/collectfiles/actions/workflows/rust.yml/badge.svg
[crates-badge]: https://img.shields.io/crates/v/collectfiles.svg?labelColor=383636
[license-badge]: https://img.shields.io/crates/l/collectfiles?labelColor=383636
[twitter-badge]: https://img.shields.io/twitter/follow/do_halee?style=flat&logo=twitter&color=4a4646&labelColor=333131&label=just-do-halee

[ci-url]: https://github.com/just-do-halee/collectfiles/actions
[twitter-url]: https://twitter.com/do_halee
[crates-url]: https://crates.io/crates/collectfiles
[license-url]: https://github.com/just-do-halee/collectfiles
| [Docs](https://docs.rs/collectfiles) | [Latest Note](https://github.com/just-do-halee/collectfiles/blob/main/CHANGELOG.md) |

```toml
[dependencies]
collectfiles = "1.1.0"
```

---

# Example
```rust
use collectfiles::*;

let vec = CollectFiles("/Users/hwakyeom/programs/")
        .with_depth(1)
        .with_target_regex(".md$")
        .with_hook(|path| path.with_extension("mutated"))
        .with_unwrap_or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                PathBuf::from("/Users/other/")
            } else {
               panic!("{:?}", e)
            }
        })
        .collect();

println!("{:#?}", vec);
```
