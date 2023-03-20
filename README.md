A simple macro for counting a number of comma-separated tokens at compile time

Code originates from the little book of rust macros by DanielKeep. I thought i'd add this as a crate to crates.io so it can easily be imported to any project if needed. No std required. I find it very useful for counting the number of elements when implementing traits for tuples within a macro.

Original macro code: https://danielkeep.github.io/tlborm/book/blk-counting.html