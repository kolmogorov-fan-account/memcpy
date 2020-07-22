use criterion::{criterion_group, criterion_main, Criterion};
use memmap::MmapMut;
use bytesize::ByteSize;

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
///                     Zen 2               Coffee Lake
/// mov 32         234 ns (100 %)          135 ns (100 %)
/// mov 64          80 ns ( 34 %)           89 ns ( 66 %)
/// mov 128         63 ns ( 27 %)           39 ns ( 29 %)
/// mov 256         15 ns (  7 %)           20 ns ( 15 %)
/// mov 128 (nt)   111 ns ( 47 %)           63 ns ( 47 %)
/// mov 256 (nt)   111 ns ( 47 %)           55 ns ( 41 %)
/// rep movsb       29 ns ( 12 %)           23 ns ( 17 %)
/// rep movsq       29 ns ( 12 %)           23 ns ( 17 %)
///
///                                 4 MiB
///                     Zen 2               Coffee Lake
/// mov 32         328 μs (100 %)          321 μs (100 %)
/// mov 64         168 μs ( 51 %)          190 μs ( 59 %)
/// mov 128         82 μs ( 25 %)          146 μs ( 45 %)
/// mov 256         77 μs ( 23 %)          166 μs ( 52 %)
/// mov 128 (nt)   223 μs ( 68 %)          171 μs ( 53 %)
/// mov 256 (nt)   224 μs ( 68 %)          165 μs ( 51 %)
/// rep movsb      340 μs (103 %)          163 μs ( 51 %)
/// rep movsq      345 μs (105 %)          162 μs ( 50 %)
///
///                                 1 GiB
///                     Zen 2               Coffee Lake
/// mov 32         167 ms (100 %)          141 ms (100 %)
/// mov 64         192 ms (115 %)          114 ms ( 81 %)
/// mov 128        161 ms ( 96 %)          106 ms ( 75 %)
/// mov 256        143 ms ( 86 %)          108 ms ( 77 %)
/// mov 128 (nt)    85 ms ( 51 %)           80 ms ( 57 %)
/// mov 256 (nt)    82 ms ( 49 %)          100 ms ( 71 %)
/// rep movsb      163 ms ( 98 %)          128 ms ( 91 %)
/// rep movsq      163 ms ( 98 %)          128 ms ( 91 %)
fn run_benchmark_memcpy(c: &mut Criterion, size: ByteSize) {
    let raw_size = size.as_u64() as usize;
    let mut run_benchmark = |name: &str, memcpy: fn(usize, *mut u8, *mut u8)| {
        let mut source = MmapMut::map_anon(raw_size).unwrap();
        let mut destination = MmapMut::map_anon(raw_size).unwrap();
        c.bench_function(&format!("[{}] {}", size.to_string_as(true), name), |b| {
            b.iter(|| memcpy(raw_size, source.as_mut_ptr(), destination.as_mut_ptr()))
        });
    };

    run_benchmark("mov 32", memcpy::memcpy_mov_32);
    run_benchmark("mov 64", memcpy::memcpy_mov_64);
    run_benchmark("mov 128", memcpy::memcpy_mov_128);
    #[cfg(target_feature = "avx")]
    run_benchmark("mov 256", memcpy::memcpy_mov_256);
    run_benchmark("mov 128 (nt)", memcpy::memcpy_mov_128_nt);
    #[cfg(target_feature = "avx")]
    run_benchmark("mov 256 (nt)", memcpy::memcpy_mov_256_nt);
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
