use std::io::Write;

#[inline(never)]
pub fn write_bytes_dyn(w: &mut dyn Write, buf: &[u8]) {
    // let _span = tracy_client::span!("write_bytes_dyn");
    // std::thread::sleep(std::time::Duration::from_millis(1_000));

    // Intentionally naive byte-at-a-time writes.
    for &b in buf {
        // ignore errors for simplicity in this repro
        let _ = w.write_all(&[b]);
    }
}

#[inline(never)]
pub fn write_bytes_generic<W: Write>(mut w: W, buf: &[u8]) {
    // let _span = tracy_client::span!("write_bytes_generic");
    // std::thread::sleep(std::time::Duration::from_millis(1_000));

    for &b in buf {
        let _ = w.write_all(&[b]);
    }
}

fn main() {
    // let _tracy = tracy_client::Client::start();
    // tracy_client::set_thread_name!("intra_single_crate");
    // std::thread::sleep(std::time::Duration::from_millis(5_000));

    let n = 1024 * 1024;
    let data = vec![1u8; n];

    // Case 1: dyn Write in the same crate
    let mut v1: Vec<u8> = Vec::with_capacity(n);
    write_bytes_dyn(&mut v1, &data);
    assert_eq!(v1.len(), n);

    // Case 2: generic Write in the same crate (monomorphized)
    let mut v2: Vec<u8> = Vec::with_capacity(n);
    write_bytes_generic(&mut v2, &data);
    assert_eq!(v2.len(), n);

    // Sanity
    assert_eq!(v1, v2);
    println!("ok:{} bytes", v1.len());
}
