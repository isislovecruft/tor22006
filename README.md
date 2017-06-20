# tor22006

Benchmark different designs for https://bugs.torproject.org/22006.

# results

    ∃!isisⒶwintermute:(master +)~/code/rust/tor22006 ∴ cargo bench --features="bench"
       Compiling tor22006 v0.1.0 (file:///home/isis/code/rust/tor22006)
        Finished release [optimized] target(s) in 1.3 secs
         Running target/release/deps/tor22006-4d45ae24d96b617f

    running 2 tests
    test bench::current_design     ... bench:     113,027 ns/iter (+/- 31,722)
    test bench::with_decaf_instead ... bench:       7,817 ns/iter (+/-  3,706)

    test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
