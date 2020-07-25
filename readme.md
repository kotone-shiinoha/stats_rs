# Overview
stats_rs is a statistics library for rust lang.

# Is it production ready?
No. Not every functions are tested.

# FYI
## No Validation/Sorting
Although this library is inspired by go lang's stats library, functions that this library provide will not make a sorted copy of arguments that it receives before processing. Nor it does validate.

Functions like `median` or `linear_regression` will return could return a wrong value.  

## Ordering
`Ord` trait is crucial for some of the algorithm used in the library, however, f64 doesn't have it implemented. 
Thus, the library will use `OrdFloat` when its necessary

## functions

# Todo
- better document
- better code organization
- better names for modules/functions
- support generic arguments