use criterion::{black_box, Criterion, criterion_group, criterion_main};
use std::path::Path;

fn bench_decode_png(c: &mut Criterion) {
    let path = Path::new("tests/fixtures/valid.png");
    if !path.exists() {
        eprintln!("Skipping bench: tests/fixtures/valid.png not found");
        return;
    }
    c.bench_function("decode_png", |b| {
        b.iter(|| {
            let image = image::ImageReader::open(black_box(path))
                .expect("open")
                .with_guessed_format()
                .expect("format")
                .decode()
                .expect("decode");
            let _ = black_box(image);
        })
    });
}

fn bench_decode_jpeg(c: &mut Criterion) {
    let path = Path::new("tests/fixtures/valid.jpg");
    if !path.exists() {
        eprintln!("Skipping bench: tests/fixtures/valid.jpg not found");
        return;
    }
    c.bench_function("decode_jpeg", |b| {
        b.iter(|| {
            let image = image::ImageReader::open(black_box(path))
                .expect("open")
                .with_guessed_format()
                .expect("format")
                .decode()
                .expect("decode");
            let _ = black_box(image);
        })
    });
}

criterion_group!(benches, bench_decode_png, bench_decode_jpeg);
criterion_main!(benches);
