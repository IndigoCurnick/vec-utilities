# vec-utilities

Rust crate to import high-level statistical utilities on `Vec` as a trait. For 
example 

- mean
- mode
- median
- range
- standard deviation

and more.

As convention, this crate provides two versions of each function. If prepended 
by `nan_`, that function will effectively ignore any `NaN` in the `Vec`.  