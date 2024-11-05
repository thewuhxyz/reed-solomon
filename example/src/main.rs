#[macro_use(shards)]
extern crate reed_solomon_erasure;

use reed_solomon_erasure::galois_8::ReedSolomon;

fn main() {
    let r = ReedSolomon::new(3, 4).unwrap(); // 3 data shards, 2 parity shards

    let mut master_copy = shards!(
        [0, 4, 2, 3],
        [4, 5, 6, 7],
        [8, 10, 10, 11],
        [0, 0, 0, 0], // last 2 rows are parity shards
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    );

    // Construct the parity shards
    r.encode(&mut master_copy).unwrap();

    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    let mut shards: Vec<_> = master_copy.iter().cloned().map(Some).collect();

    // We can remove up to 2 shards, which may be data or parity shards
    shards[0] = None;
    shards[1] = None;
    shards[2] = None;

    println!("shards: {:?}", shards);

    // Try to reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();

    // Convert back to normal shard arrangement
    let result: Vec<_> = shards.into_iter().flatten().collect();

    println!("rpo: {:?}", result);

    assert!(r.verify(&result).unwrap());
    assert_eq!(master_copy, result);
}
