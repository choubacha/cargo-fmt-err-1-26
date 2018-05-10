# Cargo fmt bug

It would seem that `r` string literals inside a macro appear to indent
each time cargo fmt is run. Check this codebase out and try running
cargo fmt as of v1.26
