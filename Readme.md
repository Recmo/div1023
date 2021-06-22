# Dividide by 1023

## Benchmark

```
cargo run --release
```

```
div1023/reference       time:   [77.923 ns 78.617 ns 79.366 ns]
div1023/tuned           time:   [77.140 ns 77.758 ns 78.454 ns]
div1023/asm             time:   [75.786 ns 76.361 ns 76.992 ns]
div1023/mgdiv           time:   [52.360 ns 52.751 ns 53.201 ns]
```
