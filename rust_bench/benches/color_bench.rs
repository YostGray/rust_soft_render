#[macro_use]
extern crate criterion;
extern crate rand;
extern crate rust_tiny_img;

use criterion::Criterion;
use rand::{thread_rng, Rng};
use rust_tiny_img::color::Color;

fn bench_to_gray(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("Color to gray", move |b| {
        b.iter(|| Color::new(rng.gen_range(0..=255),rng.gen_range(0..=255),rng.gen_range(0..=255),255).color_to_gray())
    });
}

fn bench_liner_inter(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("linear interpolation", move |b| {
        let color_a = Color::new(rng.gen_range(0..=255),rng.gen_range(0..=255),rng.gen_range(0..=255),255);
        let color_b = Color::new(rng.gen_range(0..=255),rng.gen_range(0..=255),rng.gen_range(0..=255),255);
        let mix_rate = rng.gen_range(0.0..1.0);
        b.iter(|| Color::linear_interpolation(&color_a,&color_b,mix_rate))
    });
}

criterion_group!(
    benches,
    bench_to_gray,
    bench_liner_inter,
);

criterion_main!(benches);