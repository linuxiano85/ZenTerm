# Warp-like Terminal Bootstrap (Rust)

Base di progetto Rust per un terminale stile Warp.

## Struttura

- `apps/desktop`: binario desktop (winit), pronto per integrare wgpu/egui.
- `crates/engine`: libreria core (per parser terminale, buffer, ecc.) — placeholder.
- `tests/`: test di fumo a livello workspace.

## Primi passi

```bash
# Build e test
cargo build
cargo test

# Lanciare l'app
cargo run -p app-desktop
```

## CI

Workflow GitHub Actions `Rust CI`:
- `fmt` (rustfmt)
- `clippy` (warning = error)
- build e test su Rust stable
