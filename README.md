[![github]](https://github.com/usagi/ToUnit)&ensp;[![crates-io]](https://crates.io/crates/ToUnit)&ensp;[![docs-rs]](https://docs.rs/ToUnit)<br>
[![Build Status](https://travis-ci.org/usagi/ToUnit.svg?branch=master)](https://travis-ci.org/usagi/ToUnit)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

# ToUnit; drop to the trash syntax sugar for any types.

## Example

```rust
use to_unit::ToUnit;
// or, `use to_unit::ToUnit as __` for ultra lzay persons!

#[test]
fn match_arms_to_easily()
{
 use std::collections::HashMap;
 let mut x = HashMap::<String, i32>::new();
 x.insert("neko".to_string(), 123);
 match x.get_mut("neko")
 {
  Some(v) => *v = 222,
  None => x.insert("neko".to_string(), 222).to_unit() // <-- here!
 }
 // Ofcorse alternatively, you can write:
 //  eg. None => { x.insert("neko".to_string(), 222); }
 // But, it might be fix to a multi-line format by rust-fmt then...:
 //  eg. None => {
 //          x.insert("neko".to_string(), 222);
 //      }
 // I don't like the multi-line behaviors, so I made the __ lib.
 //
}
```

## LICENSE

- [MIT](LICENSE.md)

## Author

- USAGI.NETWORK / Usagi Ito <https://github.com/usagi/>
