# Warp-like Terminal Bootstrap (Rust)

Base di progetto Rust per un terminale stile Warp.

## Struttura

- `apps/desktop`: binario desktop con TUI interattiva
- `crates/engine`: libreria core (config, GPU, UI)
- `tests/`: test di fumo a livello workspace.

## Primi passi

```bash
# Build e test
cargo build
cargo test

# Lanciare l'app
cargo run -p app-desktop
```

## TUI (Terminal User Interface)

ZenTerm fornisce un'interfaccia TUI interattiva costruita con `ratatui` e `crossterm`.

### Prima esecuzione: Wizard Setup

Al primo avvio, ZenTerm mostra un wizard di setup a 3 passi:

1. **Tema** - Scegli tra dark/light
2. **Limite GPU** - Imposta percentuale limite (25%, 50%, 75%, 100%)  
3. **Telemetria** - Abilita/disabilita raccolta dati anonimi

### Schermata Runtime

Dopo il setup, ZenTerm mostra:
- Header con stato corrente (tema, GPU, telemetria)
- Area principale placeholder per terminale
- Footer con scorciatoie

### Pannello Impostazioni

Accessibile tramite F2 o 's' dalla schermata runtime.
Permette di modificare tutte le impostazioni:
- Tema (dark/light toggle)
- Limite GPU (cicla 25→50→75→100→25...)
- Telemetria (on/off)
- Accelerazione GPU (on/off)

### Keybindings

#### Wizard
- `d`/`l` - Cambia tema (dark/light)
- `2`/`5`/`7`/`1` - Imposta limite GPU (25%/50%/75%/100%)
- `y`/`n` - Abilita/disabilita telemetria
- `Enter`/`Tab` - Passo successivo
- `q` - Esci (chiede conferma se wizard incompleto)

#### Runtime
- `F2` o `s` - Apri pannello impostazioni
- `q` - Esci

#### Impostazioni
- `Up`/`Down` - Naviga tra opzioni
- `Enter`/`Space` - Modifica/toggle impostazione selezionata
- `Esc` - Torna alla schermata runtime
- `q` - Esci dall'applicazione

### Persistenza Config

- Le impostazioni sono salvate automaticamente con debouncing (500ms dopo l'ultima modifica)
- Salvataggio forzato all'uscita dell'applicazione
- File config: `~/.config/zenterm/config.toml`
- Ripristino terminale garantito anche in caso di panic

## CI

Workflow GitHub Actions `Rust CI`:
- `fmt` (rustfmt)
- `clippy` (warning = error)
- build e test su Rust stable
