use bytesize::ByteSize;
use criterion::{criterion_group, criterion_main, Criterion};
use memmap::MmapMut;

/// Zen 2 (AMD)
///     Desktop
///     DDR4
///     L1: 1 MiB
///     L2: 8 MiB
///     L3: 64 MiB
///
/// Coffee Lake (Intel)
///     Mobile
///     DDR4
///     L1: 64 kiB
///     L2: 256 kiB
///     L3: 16 MiB
///
///                                 2 kiB
///
///                     Zen 2               Coffee Lake
///
/// mov 32         234 ns (100 %)          120 ns (100 %)
///
/// mov 64          80 ns ( 34 %)           89 ns ( 74 %)
/// mov 128         63 ns ( 27 %)           37 ns ( 31 %)
/// mov 256         15 ns (  7 %)           20 ns ( 17 %)
///
/// mov 64 (pl)                             58 ns ( 48 %)
/// mov 128 (pl)                            30 ns ( 25 %)
/// mov 256 (pl)                            16 ns ( 25 %)
///
/// mov 64 (nt)                             81 ns ( 68 %)
/// mov 128 (nt)   111 ns ( 47 %)           64 ns ( 53 %)
/// mov 256 (nt)   111 ns ( 47 %)           54 ns ( 45 %)
///
/// mov 64 (p+n)                            81 ns ( 49 %)
/// mov 128 (p+n)  111 ns ( 47 %)           54 ns ( 45 %)
/// mov 256 (p+n)  111 ns ( 47 %)           54 ns ( 45 %)
///
/// rep movsb       29 ns ( 12 %)           22 ns ( 18 %)
/// rep movsq       29 ns ( 12 %)           23 ns ( 19 %)
///
///                                 4 MiB
///
///                     Zen 2               Coffee Lake
///
/// mov 32         328 μs (100 %)          306 μs (100 %)
///
/// mov 64         168 μs ( 51 %)          245 μs ( 80 %)
/// mov 128         82 μs ( 25 %)          146 μs ( 48 %)
/// mov 256         77 μs ( 23 %)          136 μs ( 44 %)
///
/// mov 64 (p)                             177 μs ( 58 %)
/// mov 128 (p)                            146 µs ( 48 %)
/// mov 256 (p)                            134 µs ( 44 %)
///
/// mov 64 (n)                             173 μs ( 57 %)
/// mov 128 (n)    223 μs ( 68 %)          173 μs ( 57 %)
/// mov 256 (n)    224 μs ( 68 %)          168 μs ( 55 %)
///
/// mov 64 (p+n)                           197 ns ( 64 %)
/// mov 128 (p+n)  111 ns ( 47 %)          178 ns ( 58 %)
/// mov 256 (p+n)  111 ns ( 47 %)          179 ns ( 58 %)
///
/// rep movsb      340 μs (103 %)          167 μs ( 55 %)
/// rep movsq      345 μs (105 %)          211 μs ( 68 %)
///
///                                 1 GiB
///
///                     Zen 2               Coffee Lake
///
/// mov 32         167 ms (100 %)          145 ms (100 %)
///
/// mov 64         192 ms (115 %)          121 ms ( 83 %)
/// mov 128        161 ms ( 96 %)          106 ms ( 73 %)
/// mov 256        143 ms ( 86 %)          112 ms ( 77 %)
///
/// mov 64 (p)                             115 ms ( 79 %)
/// mov 128 (p)                            108 ms ( 74 %)
/// mov 256 (p)                            111 ms ( 77 %)
///
/// mov 64 (n)                              87 ms ( 60 %)
/// mov 128 (n)                             80 ms ( 55 %)
/// mov 256 (n)                             98 ms ( 68 %)
///
/// mov 64 (p+n)                            86 ms ( 59 %)
/// mov 128 (p+n)                           81 ms ( 56 %)
/// mov 256 (p+n)                          102 ms ( 70 %)
///
/// rep movsb      163 ms ( 98 %)          129 ms ( 89 %)
/// rep movsq      163 ms ( 98 %)          128 ms ( 88 %)
fn run_benchmark_memcpy(c: &mut Criterion, size: ByteSize) {
    let raw_size = size.as_u64() as usize;
    let mut run_benchmark = |name: &str, memcpy: fn(usize, *mut u8, *mut u8)| {
        let mut source = MmapMut::map_anon(raw_size).unwrap();
        let mut destination = MmapMut::map_anon(raw_size).unwrap();
        memcpy(raw_size, source.as_mut_ptr(), destination.as_mut_ptr());
        c.bench_function(&format!("[{}] {}", size.to_string_as(true), name), |b| {
            b.iter(|| memcpy(raw_size, source.as_mut_ptr(), destination.as_mut_ptr()))
        });
    };

    run_benchmark("mov 32", memcpy::memcpy_mov_32);
    run_benchmark("mov 64", memcpy::memcpy_mov_64);
    run_benchmark("mov 128", memcpy::memcpy_mov_128);
    #[cfg(target_feature = "avx")]
    run_benchmark("mov 256", memcpy::memcpy_mov_256);
    run_benchmark("mov 64 (pl)", memcpy::memcpy_mov_64_pl);
    run_benchmark("mov 128 (pl)", memcpy::memcpy_mov_128_pl);
    #[cfg(target_feature = "avx")]
    run_benchmark("mov 256 (pl)", memcpy::memcpy_mov_256_pl);
    run_benchmark("mov 64 (nt)", memcpy::memcpy_mov_64_nt);
    run_benchmark("mov 128 (nt)", memcpy::memcpy_mov_128_nt);
    #[cfg(target_feature = "avx")]
    run_benchmark("mov 256 (nt)", memcpy::memcpy_mov_256_nt);
    run_benchmark("mov 64 (nt+pl)", memcpy::memcpy_mov_64_nt_pl);
    run_benchmark("mov 128 (nt+pl)", memcpy::memcpy_mov_128_nt_pl);
    #[cfg(target_feature = "avx")]
    run_benchmark("mov 256 (nt+pl)", memcpy::memcpy_mov_256_nt_pl);
    run_benchmark("rep movsb", memcpy::memcpy_rep_movsb);
    run_benchmark("rep movsq", memcpy::memcpy_rep_movsq);
}

fn benchmark_memcpy_2kib(c: &mut Criterion) {
    run_benchmark_memcpy(c, ByteSize::kib(2));
}

fn benchmark_memcpy_4mib(c: &mut Criterion) {
    run_benchmark_memcpy(c, ByteSize::mib(4));
}

fn benchmark_memcpy_1gib(c: &mut Criterion) {
    run_benchmark_memcpy(c, ByteSize::gib(1));
}

criterion_group! {
    name = benchmark_memcpy;
    config = Criterion::default();
    targets = benchmark_memcpy_2kib, benchmark_memcpy_4mib, benchmark_memcpy_1gib
}

criterion_main!(benchmark_memcpy);
