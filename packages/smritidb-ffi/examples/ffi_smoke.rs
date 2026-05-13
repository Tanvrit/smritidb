use smritidb_ffi::*;

fn main() {
    println!("spec_version() = {}", spec_version());
    let a = random_hv(b"hello".to_vec(), 8192);
    let b = random_hv(b"world".to_vec(), 8192);
    println!("sim(a, a) = {:.4}", similarity(a.clone(), a.clone()).unwrap());
    println!("sim(a, b) = {:.4}", similarity(a.clone(), b.clone()).unwrap());
    let bound = bind(a.clone(), b.clone()).unwrap();
    let recovered = unbind(bound, b.clone()).unwrap();
    println!("bind/unbind round-trip sim = {:.4}", similarity(a, recovered).unwrap());

    let store = Store::new(8192).unwrap();
    let _id1 = store.put("alpha".into(), b"alpha".to_vec(), vec!["test".into()]).unwrap();
    let _id2 = store.put("beta".into(), b"beta".to_vec(), vec![]).unwrap();
    let hits = store.recall("alpha".into(), 3, 0.0).unwrap();
    for h in &hits {
        println!("hit: {} sim={:.4}", String::from_utf8_lossy(&h.value), h.similarity);
    }
}
