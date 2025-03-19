use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dicedb_rs::commands::ExpireOption;

fn criterion_benchmark(c: &mut Criterion) {
    let mut client = dicedb_rs::client::Client::new("localhost".to_string(), 7379).unwrap();
    let key = "benchkey";
    c.bench_function("decr", |b| b.iter(|| client.decr(black_box(key)).is_ok()));
    c.bench_function("decrby", |b| {
        b.iter(|| client.decrby(black_box(key), black_box(2)).is_ok())
    });
    c.bench_function("del", |b| b.iter(|| client.del(black_box(key)).is_ok()));
    c.bench_function("echo", |b| {
        b.iter(|| client.echo(black_box("hello")).is_ok())
    });
    c.bench_function("exists", |b| {
        b.iter(|| client.exists(black_box(key), black_box(vec![])).is_ok())
    });
    c.bench_function("expire", |b| {
        b.iter(|| {
            client
                .expire(black_box(key), black_box(1), black_box(ExpireOption::None))
                .is_ok()
        })
    });
    c.bench_function("expireat", |b| {
        b.iter(|| {
            client
                .expireat(
                    black_box(key),
                    black_box(1),
                    black_box(dicedb_rs::commands::ExpireAtOption::None),
                )
                .is_ok()
        })
    });
    c.bench_function("expiretime", |b| {
        b.iter(|| client.expiretime(black_box(key)).is_ok())
    });
    c.bench_function("get", |b| b.iter(|| client.get(black_box(key)).is_ok()));

    c.bench_function("getdel", |b| {
        b.iter(|| client.getdel(black_box(key)).is_ok())
    });

    c.bench_function("getex", |b| {
        b.iter(|| {
            client
                .getex(
                    black_box(key),
                    black_box(dicedb_rs::commands::GetexOption::PERSIST),
                )
                .is_ok()
        })
    });

    c.bench_function("incr", |b| b.iter(|| client.incr(black_box(key)).is_ok()));
    c.bench_function("incrby", |b| {
        b.iter(|| client.incrby(black_box(key), black_box(2)).is_ok())
    });
    c.bench_function("ping", |b| b.iter(|| client.ping().is_ok()));

    c.bench_function("set", |b| {
        b.iter(|| client.set(black_box(key), black_box(1)).is_ok())
    });
    c.bench_function("setex", |b| {
        b.iter(|| {
            client
                .setex(
                    black_box(key),
                    black_box(1),
                    black_box(dicedb_rs::commands::SetOption::None),
                )
                .is_ok()
        })
    });
    c.bench_function("ttl", |b| b.iter(|| client.ttl(black_box(key)).is_ok()));
    c.bench_function("type", |b| b.iter(|| client.dtype(black_box(key)).is_ok()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
