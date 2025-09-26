
# rust-devirt-bufio-repro

Minimal workspace to reproduce intra-crate vs cross-crate optimization differences for buffered I/O style loops with `dyn Write` vs generic `Write`.

It contains:
- `crates/intra_single_crate`: single binary crate with two functions:
  - `write_bytes_dyn(&mut dyn Write, &[u8])` (trait object, same crate)
  - `write_bytes_generic<W: Write>(W, &[u8])` (monomorphized generic)
- `crates/inter_two_crates/writerlib`: library crate exporting `write_bytes_dyn_lib` that takes `&mut dyn Write`.
- `crates/inter_two_crates/app`: binary crate that calls into the library.

The goal: compare code generation and runtime between:
1. **Intra‑crate `dyn`** vs **generic** in one crate.
2. **Cross‑crate `dyn`** (lib + app) with LTO off vs ThinLTO vs codegen‑units=1.

## Build and run

Use `cargo-show-asm` invoked as `cargo asm` to get the assembly readouts. Try toggling LTO=off, ThinLTO, codegen-units=1, etc.
