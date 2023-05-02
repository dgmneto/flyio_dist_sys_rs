Output from latest run:
```
INFO [2023-05-02 21:52:31,336] jepsen test runner - jepsen.core {:net {:all {:send-count 20150,
             :recv-count 20150,
             :msg-count 20150,
             :msgs-per-op 25.668789},
       :servers {:send-count 18480,
                 :recv-count 18480,
                 :msg-count 18480,
                 :msgs-per-op 23.5414}},
 :workload {:stable-latencies {0 0, 0.5 0, 0.95 0, 0.99 0, 1 0}}}
```

The [requirements](https://fly.io/dist-sys/3d/) are:
- max msgs-per-op 30 (actual 25.668789)
- p50 latency 400 (actual 0)
- max latency 600 (acutal 0)

This was achieved by not blocking on any x-server message exchange.

This won't work for the partition dataset.