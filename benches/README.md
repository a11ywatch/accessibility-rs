# benchmarks

Benchmarking the accessibility engine.

## Stages

The performance is set at stage 0 of 3 more details of the benchmark goals. Each stage should have major performance increases.

```sh
# stage 0
audit-speed/core/audit: small html (4k iterations)
time: [66.025 µs 66.151 µs 66.270 µs]
audit-speed/core/audit: medium html (4k iterations)
time: [928.16 µs 931.17 µs 933.96 µs]
audit-speed/core/audit: large html (4k iterations)                                                                            
time: [1.1475 ms 1.1507 ms 1.1531 ms]
```