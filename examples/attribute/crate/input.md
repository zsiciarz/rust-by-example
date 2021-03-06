The `crate_type` attribute can be used to tell the compiler whether a crate is
a binary or a library (and even which type of library). And the `crate_id`
attribute can be used to set the name and the version of the crate.

{lib.rs}

When the `crate_type` attribute is used, we no longer need to pass the
`--crate-type` flag to `rustc`.

```
$ rustc lib.rs
$ ls lib*
liberty-a1e7dc98-0.1.rlib
```
