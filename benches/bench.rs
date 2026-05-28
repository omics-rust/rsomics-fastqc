use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_fastqc(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-fastqc");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fq = manifest.join("tests/golden/tiny.fq");
    c.bench_function("rsomics-fastqc golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args([fq.to_str().unwrap(), "--stdout"])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_fastqc);
criterion_main!(benches);
