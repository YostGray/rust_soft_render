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
    c.bench_function("Color to gray", move |b| {
        b.iter(|| Color::new(rng.gen_range(0, 255),rng.gen_range(0, 255),rng.gen_range(0, 255)).color_to_gray())
    });
}

fn bench2(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("linear interpolation", move |b| {
        let color_a = Color::new(rng.gen_range(0, 255),rng.gen_range(0, 255),rng.gen_range(0, 255));
        let color_b = Color::new(rng.gen_range(0, 255),rng.gen_range(0, 255),rng.gen_range(0, 255));
        let mix_rate = rng.gen_range(0.0, 1.0);
        b.iter(|| Color::linear_interpolation(&color_a,&color_b,mix_rate))
    });
}

criterion_group!(
    benches,
    // bench1,
    bench2,
);

criterion_main!(benches);