use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg64;

use i2d_tree::Item;
use i2d_tree::Node;


fn build_tree(len: usize, seed: u64) -> Node<usize> {
    let mut items = Vec::new();
    let mut rng = Pcg64::seed_from_u64(seed);

    for i in 0..len {
        let latitude = rng.gen_range(50.0..60.0);
        let longitude = rng.gen_range(30.0..40.0);

        items.push(Item::new(latitude, longitude, i));
    }

    Node::build(&mut items)
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("build_tree 100", |b| b.iter(|| build_tree(black_box(100), 3)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);