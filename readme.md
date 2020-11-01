# Overview
stats_rs is a statistics library for rust lang.

# Is it production ready?
No. Not every functions are tested.

# FYI
## No Validation/Sorting
Although this library is inspired by go lang's stats library, functions in this library will neither validate nor make a sorted copy of arguments that you pass.

## Ordering
`Ord` trait is crucial for some of the algorithm used in the library but f64 doesn't have it implemented.
To overcome with the problem, this library will use `OrdFloat` behind the hood when its necessary

# Todo
- better document
- better code organization
- better names for modules/functions
- support generic arguments
