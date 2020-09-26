# levenshtein

Making rust as fast as **it can** go

Improving the performance of the Rust code in [Christian's article](https://www.christianfscott.com/making-rust-as-fast-as-go/)

- Baseline: 2.077593106
- [6ff99a5](https://github.com/anderj017/levenshtein/commit/6ff99a58495c93829b4c300371e87d0262478952): &str.chars().count() -> &str.len(): 2.022744823
- [1dede2f](https://github.com/anderj017/levenshtein/commit/1dede2fb13795fe57e29946b59adb93265e1807e): reuse same vector: 1.880497681
- [6adaf13](https://github.com/anderj017/levenshtein/commit/6adaf138debe7a6c55fdb9118ffe161f4ff89bc5): build cache without .collect(): 1.7518643919999999

And breaking some of Christian's rules:

- [92d0b82](https://github.com/anderj017/levenshtein/commit/92d0b82d9758f64237057b3f2d15c40423209a22): remove vector bounds checks: 1.475065986
- [23a7604](https://github.com/anderj017/levenshtein/commit/23a7604daab09470f0da65c79ae3d90d02470f2a): pre-allocate vector 100 chars: 1.455840814
- [7f47ea9](https://github.com/anderj017/levenshtein/commit/7f47ea9d512e2dc39be6167148cb1f9a768559c4): add notes of some interesting findings
- [73bdea8](https://github.com/anderj017/levenshtein/commit/73bdea8b875bac3c8ab1e5cdd80ef20fef15f42b): use Wrapping to avoid int overflow checks (no real difference)
- [03f9200](https://github.com/anderj017/levenshtein/commit/03f9200541e42c0424f029579c9e3037bee66c64): eliminate branches (no real difference)

Interesting findings:

- Vector bounds checks are surprisingly expensive
- Rust's opt-in performance improvements look rather ugly :(
- `std::cmp::min` has poor performance compared to a hand written function (which was included in the Baseline). This is rather curious and concerning.
