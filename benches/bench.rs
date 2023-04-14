mod data;
use std::num::NonZeroUsize;
use data::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use k_means::{cluster_v1, cluster_v2, cluster_v3, cluster_v4};

fn criterion_benchmark(c: &mut Criterion) {
    let data = read_data("data/iris-10000.csv").unwrap_or_else(|_| panic!("не удалось найти data/iris-10000.csv)"));
    let tests = (1usize..=10).map(|x| 1000*x);
    let tests = [10000];
    let mut group = c.benchmark_group("cluster iris");
    for size in tests {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("v1",size), &size, |b, &size| {
            b.iter(|| cluster_v1(&data[0..size], NonZeroUsize::new(3).unwrap()));
        });
        group.bench_with_input(BenchmarkId::new("v2",size), &size, |b, &size| {
            b.iter(|| cluster_v2(&data[0..size], NonZeroUsize::new(3).unwrap()));
        });
        group.bench_with_input(BenchmarkId::new("v3",size), &size, |b, &size| {
            b.iter(|| cluster_v3(&data[0..size], NonZeroUsize::new(3).unwrap()));
        });
        group.bench_with_input(BenchmarkId::new("v4",size), &size, |b, &size| {
            b.iter(|| cluster_v4(&data[0..size], NonZeroUsize::new(3).unwrap()));
        });
    }
    println!("file:///C:/Users/kondratiev/dev/k-means/target/criterion/cluster%20iris/report/index.html")
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);