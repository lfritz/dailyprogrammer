# 2015-05-20] Challenge #215 [Intermediate] Validating sorting networks

Solutions for [Challenge #215 Intermediate](http://www.reddit.com/r/dailyprogrammer/comments/36m83a) in Rust.

I tried to make it fast by avoiding branches in the inner loops. To see if that actually helps, I also made a version with `if` statements instead of boolean logic -- that's `validate-ifs.rs`. It is indeed somewhat slower, although not that much (about 20 percent with the `challenge-2.txt` dataset).
