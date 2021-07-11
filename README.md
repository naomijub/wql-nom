# [WooriDB](https://github.com/naomijub/wooridb) Query Language parser

* parsing is done with parse combinators crate [`nom`](https://github.com/Geal/nom)

This project was interesting to learn nom, but wooridb/wql pure implementation is quite faster:

WQL-NOM:
```
create_entity           time:   [937.71 ns 943.35 ns 949.65 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

Benchmarking inser_entity: Collecting 100 samples in estimated 5.0023 s (662k it                                                                                inser_entity            time:   [7.3289 us 7.3695 us 7.4135 us]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

WQL pure:
```
create_entity           time:   [814.34 ns 817.61 ns 821.37 ns]
                        change: [-99.985% -99.985% -99.984%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

Benchmarking inser_entity: Collecting 100 samples in estimated 5.0117 s (1.6M it                                                                                inser_entity            time:   [3.0017 us 3.0162 us 3.0335 us]
                        change: [-0.9309% -0.3018% +0.3123%] (p = 0.35 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
```