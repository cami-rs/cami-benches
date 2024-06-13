# Benchmarking cami

## Scope/Summary

- Linux, and at least initially: major x64 distributions
- at least initially: major Rust targets (`x86_64-unknown-linux-gnu`)
- Valgrind + Crabgrind.

### Physical/kernel counters

This is hard, especially in CI/cloud environment. But it's hard locally, too. Even on dedicated
hardware, you'd need to
- limit other applications/processes and any system/package manager/regular tasks - possibly with
  cgroups/`systemd-run`, and
- prevent overheating
  - limit CPU speed, so that the CPU doesn't overheat (`cpupower` & `cpupower-gui` on Intel)
  - and/or turn off CPU core(s) that you don't need,
  - and/or assign system/package manager/regular tasks to core(s) that you throttle (with `taskset`
    or cgroups/`systemd-run`)
- but, don't slow the CPU too much, otherwise, if its speed is comparable to RAM speed, the
  cache-friendly operation will not show any benefit.

On development machines that's even less reliable (or more difficult) due to the window manager,
garbage collection...

Feel free to share practical examples/steps - rather than bits and pieces, and pull requests. But,
until then we're not doing this.

### Valgrind

For now.

## Which crates - "tiers"

`cami` is primarily benchmarked for `core`/`std` types (slices, `&str`, `String`, `Vec`) and for
examples of compound types.

Feel free to add benchmarks for major 3rd party types, if you're willing to maintain them.
