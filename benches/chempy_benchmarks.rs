use chempy::molecule::{Molecule, Atom, Bond, BondOrder};
use chempy::element;
use chempy::kinetics::{ArrheniusModel, KineticsModel};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn setup_benzene() -> Molecule {
    let mut benzene = Molecule::new();
    let carbons: Vec<usize> = (0..6).map(|_| benzene.add_atom(Atom::new(&element::C))).collect();
    for i in 0..6 {
        let order = if i % 2 == 0 { BondOrder::Double } else { BondOrder::Single };
        benzene.add_bond(carbons[i], carbons[(i + 1) % 6], Bond::new(order));
    }
    benzene
}

fn bench_isomorphism(c: &mut Criterion) {
    let benzene1 = setup_benzene();
    let benzene2 = setup_benzene();
    
    c.bench_function("isomorphism_benzene", |b| {
        b.iter(|| benzene1.is_isomorphic(black_box(&benzene2)))
    });
}

fn bench_kinetics(c: &mut Criterion) {
    let model = ArrheniusModel::new(1.0e10, 0.5, 50000.0, 1.0);
    let t = 1000.0;
    let p = 1.0e5;
    
    c.bench_function("kinetics_arrhenius", |b| {
        b.iter(|| model.get_rate_coefficient(black_box(t), black_box(p)))
    });
}

criterion_group!(benches, bench_isomorphism, bench_kinetics);
criterion_main!(benches);
