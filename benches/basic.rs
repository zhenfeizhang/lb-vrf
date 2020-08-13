#[macro_use]
extern crate criterion;

use criterion::Benchmark;
use criterion::Criterion;
use lb_vrf::lbvrf::LBVRF;
use lb_vrf::param::Param;
use lb_vrf::VRF;
use std::time::Duration;

criterion_group!(basic, ot_lbvrf,);
criterion_main!(basic);

fn ot_lbvrf(c: &mut Criterion) {
    let seed = [0u8; 32];
    let bench_str = format!("parameter generation");
    let bench = Benchmark::new(bench_str, move |b| {
        b.iter(|| {
            <LBVRF as VRF>::paramgen(seed).unwrap();
        });
    });

    let param: Param = <LBVRF as VRF>::paramgen(seed).unwrap();
    let bench_str = format!("key generation");
    let bench = bench.with_function(bench_str, move |b| {
        b.iter(|| {
            <LBVRF as VRF>::keygen(seed, param).unwrap();
        });
    });

    let (pk, sk) = <LBVRF as VRF>::keygen(seed, param).unwrap();
    let message = "this is a message that vrf signs";

    let bench_str = format!("proof");
    let bench = bench.with_function(bench_str, move |b| {
        b.iter(|| {
            <LBVRF as VRF>::prove(message, param, pk, sk, seed).unwrap();
        });
    });

    let bench_str = format!("verify");
    let proof = <LBVRF as VRF>::prove(message, param, pk, sk, seed).unwrap();
    let bench = bench.with_function(bench_str, move |b| {
        b.iter(|| {
            let res = <LBVRF as VRF>::verify(message, param, pk, proof).unwrap();
            assert!(res.is_some());
        });
    });

    let bench = bench.warm_up_time(Duration::from_millis(1000));
    let bench = bench.measurement_time(Duration::from_millis(5000));
    let bench = bench.sample_size(100);
    c.bench("one time lbvrf", bench);
}
