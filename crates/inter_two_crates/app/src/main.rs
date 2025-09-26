use writerlib::write_bytes_dyn_lib;

fn main() {
    // let _tracy = tracy_client::Client::start();
    // tracy_client::set_thread_name!("writerlib_app");
    // std::thread::sleep(std::time::Duration::from_millis(2_000));
    let n = 1024 * 1024;
    let data = vec![2u8; n];

    let mut v: Vec<u8> = Vec::with_capacity(n);
    // Cross-crate dyn Write call
    // let _span = tracy_client::span!("writerlib_app::write_bytes_dyn_lib");
    write_bytes_dyn_lib(&mut v, &data);
    assert_eq!(v.len(), n);
    println!("ok:{} bytes", v.len());
}
