use std::io::Write;

/// Exported from a separate crate to test cross-crate optimizations.
#[inline(never)]
pub fn write_bytes_dyn_lib(w: &mut dyn Write, buf: &[u8]) {
    std::thread::sleep(std::time::Duration::from_millis(1_000));

    for &b in buf {
        let _ = w.write_all(&[b]);
    }
}
