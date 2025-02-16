use criterion::{criterion_group, criterion_main, Criterion, BenchmarkGroup};
use kvs::{KvStore, KvsEngine, SledKvsEngine};
use rand::prelude::*;
use tempfile::TempDir;
use std::time::Duration;

fn generate_random_string(rng: &mut ThreadRng, len: usize) -> String {
    rng.sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn generate_test_data(num_pairs: usize) -> Vec<(String, String)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity(num_pairs);
    
    for _ in 0..num_pairs {
        let key_len = rng.gen_range(1, 1000); // Reduced size for faster tests
        let value_len = rng.gen_range(1, 1000);
        
        let key = generate_random_string(&mut rng, key_len);
        let value = generate_random_string(&mut rng, value_len);
        
        data.push((key, value));
    }
    
    data
}

fn configure_group<'a>(c: &'a mut Criterion, name: &str) -> BenchmarkGroup<'a, criterion::measurement::WallTime> {
    let mut group = c.benchmark_group(name);
    group.sample_size(10)
         .measurement_time(Duration::from_secs(10))
         .warm_up_time(Duration::from_secs(3));
    group
}

fn bench_write<E: KvsEngine>(group: &mut BenchmarkGroup<criterion::measurement::WallTime>, name: &str, create_engine: impl Fn() -> E) {
    group.bench_function(name, |b| {
        b.iter_with_setup(
            || {
                let store = create_engine();
                let data = generate_test_data(100);
                (store, data)
            },
            |(mut store, data)| {
                for (key, value) in data {
                    store.set(key, value).unwrap();
                }
            },
        )
    });
}

fn bench_read<E: KvsEngine>(group: &mut BenchmarkGroup<criterion::measurement::WallTime>, name: &str, create_engine: impl Fn() -> E) {
    group.bench_function(name, |b| {
        let mut store = create_engine();
        let data = generate_test_data(1000);
        
        for (key, value) in &data {
            store.set(key.clone(), value.clone()).unwrap();
        }
        
        b.iter_with_setup(
            || data.clone(),
            |data| {
                for (key, expected_value) in &data {
                    let value = store.get(key.clone()).unwrap();
                    assert_eq!(value.as_ref(), Some(expected_value));
                }
            }
        )
    });
}

fn write_benchmark(c: &mut Criterion) {
    let mut group = configure_group(c, "write_benchmarks");
    
    bench_write(&mut group, "kvs_write", || {
        let temp_dir = TempDir::new().expect("unable to create temporary working directory");
        KvStore::open(temp_dir.into_path()).unwrap()
    });
    
    bench_write(&mut group, "sled_write", || {
        let temp_dir = TempDir::new().expect("unable to create temporary working directory");
        SledKvsEngine::new(temp_dir.into_path()).unwrap()
    });
    
    group.finish();
}

fn read_benchmark(c: &mut Criterion) {
    let mut group = configure_group(c, "read_benchmarks");
    
    bench_read(&mut group, "kvs_read", || {
        let temp_dir = TempDir::new().expect("unable to create temporary working directory");
        KvStore::open(temp_dir.into_path()).unwrap()
    });
    
    bench_read(&mut group, "sled_read", || {
        let temp_dir = TempDir::new().expect("unable to create temporary working directory");
        SledKvsEngine::new(temp_dir.into_path()).unwrap()
    });
    
    group.finish();
}

criterion_group!(benches, write_benchmark, read_benchmark);
criterion_main!(benches);
