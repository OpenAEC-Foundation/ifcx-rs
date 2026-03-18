//! Parser benchmarks.
//!
//! Run with: cargo bench -p ifcx-core

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_parse_small(c: &mut Criterion) {
    let json = serde_json::to_string(&ifcx_core::IfcxDocument::new("bench")).unwrap();

    c.bench_function("parse_empty_doc", |b| {
        b.iter(|| {
            let _doc: ifcx_core::IfcxDocument = serde_json::from_str(&json).unwrap();
        });
    });
}

criterion_group!(benches, bench_parse_small);
criterion_main!(benches);
