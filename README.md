# Simple Benchmarks

## Intro

These are *very* simple benchmarks stress-testing just simple and bulk operations. No fancy n! calculations. No 
benchmark projects you need to lookup. Just a very basic question: "Can this be shoe-horned into a website to make
it faster / capable of doing more on the client side?"

This repo is inspired after benchmarking a data tree formatter in WASM:
https://github.com/jrhazlett/rust-wasm-data-formatter

I started wondering: "If I actually found a way to cut the 'Reflect' calls from the library, would it help?"

## Summary of findings

To answer the above questions: No.

WASM *can* overtake JS in performance, *but* using *any* interoperability comes at a fairly substantial expense.

In tests, each conversion tends to add around 150 ms to execution times.

## Notes about benchmarks

### Optimizations added for rust:

- 'wee_alloc' swapped out for default
- All builds ran with --release
- No other optimizations (the two above options really had the most impact during initial research)

### Explanation for times

- All benchmarks gauged via 'performance.now()'
- Browser: Chrome
- All results are x1000 (to avoid simple rounding to zero)

## Code and times

All times are 'js/wasm'. Function defs are in index.js / lib.js respectively.

### get_string_unmodified: 100.00 / 299.99

This function is a straight...
- Take value
- Shift value into a state where it would be manipulated (without changing it further)
- Return value

### get_string_combined: 99.99 / 300.00

The same as 'unmodified' except there's a one-off concatenation.

### get_array_of_strings_combined_wo_arg: 100.00 / 1599.99

- Iterate over 1K integers
- Turn each integer into a string
- Concatenated each string with "_TEST"
- Collect the results in an array
- Return array of concatenated strings

### get_array_of_strings_combined_wo_arg_serde: 99.99 / 1500.00

Similar workflow to basic 'get_array_of_strings_combined_wo_arg' except this uses serde_wasm_bindgen instead of
wasm_bindgen's built-in conversion feature.

### calculation_without_return, up to 1K: 99.99 / 100 (worst times, this fluctuates wildly between these and 0)

Iterate through 1K integers in sequential order and add their value to a count value

### calculation_without_return, up to 100K: 1100.00 / 100.00

Same as above, except the count is raised to 100K





















































