#[macro_use]
extern crate criterion;
extern crate rand;
extern crate rust_tiny_img;
extern crate rust_learn_ray_tracing;

use criterion::Criterion;
use rand::{thread_rng, Rng};
use rust_learn_ray_tracing::rt_lib::vector3::Vector3;

fn bench_devide(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("vector3 divide", move |b| {
        b.iter(|| {
            let v3 = Vector3::new(rng.gen(),rng.gen(),rng.gen());
            let _ = v3 / 2.0f64 / 2.0f64 / 2.0f64;
        })
    });
}
fn bench_devide_self(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("vector3 divide self", move |b| {
        b.iter(|| {
            let mut v3 = Vector3::new(rng.gen(),rng.gen(),rng.gen());
            v3 /= 2.0f64;
            v3 /= 2.0f64;
            v3 /= 2.0f64;
        })
    });
}

criterion_group!(
    benches,
    bench_devide,
    bench_devide_self,
);

criterion_main!(benches);