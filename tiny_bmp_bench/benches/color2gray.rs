#[macro_use]
extern crate criterion;
extern crate rand;
extern crate tiny_bmp;
use std::arch::x86_64::{_mm256_set_pd, self};

use criterion::Criterion;
use rand::{thread_rng, Rng};
use tiny_bmp::color::Color;

fn bench1(c: &mut Criterion) {
    let m : x86_64::__m256d = unsafe { _mm256_set_pd(0.2126, 0.7152, 0.0722, 0.0) };
    let mut rng = thread_rng();
    c.bench_function("Color 2 Gray", move |b| {
        // b.iter(|| Color::new(rng.gen_range(0, 255),rng.gen_range(0, 255),rng.gen_range(0, 255)).color_to_gray_AVX(m))
        b.iter(|| Color::new(rng.gen_range(0, 255),rng.gen_range(0, 255),rng.gen_range(0, 255)).color_to_gray())
    });
}

criterion_group!(
    benches,
    bench1
);

criterion_main!(benches);