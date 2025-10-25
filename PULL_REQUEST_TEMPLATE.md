# Core: Event Bus Milestone 1 – Panic Isolation, Error Handling, Metrics & Reporting

## Summary
Milestone 1 del sottosistema Event Bus: robustezza e osservabilità di base (panic isolation, reporting esiti, metrics hook, wildcard più solidi, macro di emissione, builder configurabile).

## Motivazione
Evitare crash dovuti a panics nei handler, dare visibilità su success/fail/panic e preparare estensioni future (pattern precompilation, typed events, backpressure) mantenendo compatibilità retroattiva.

## Principali Modifiche
- Builder: configurazione `catch_panics`, tracing, metrics sink.
- Panic isolation (attivo di default) con `catch_unwind`.
- `HandlerResult` (Success/Error/Panic) + `EmitReport` aggregato.
- Nuove API: `subscribe_result`, `emit_and_report`.
- Macro `event_emit!` per emissione ergonomica.
- Trait `MetricsSink` (`on_emit`, `on_handler_result`, `on_panic`) + implementazione `NoopMetricsSink` di default.
- Matcher wildcard rivisto e testato (`*` e `**`).
- Documentazione aggiornata (sezione Event Bus / README).
- Test aggiuntivi: wildcard, error handling, macro, reporting.

## API Aggiunte
- `EventBus::builder()`, `EventBusBuilder` (`catch_panics`, `tracing`, `metrics_sink`, `build`).
- `EventBus::subscribe_result(...)`
- `EventBus::emit_and_report(...) -> EmitReport`
- Struct `EmitReport { key, handlers, successes, errors, panics, fn is_all_ok() }`
- Enum `HandlerResult`
- Trait `MetricsSink` + `NoopMetricsSink`
- Macro `event_emit!`

## Compatibilità
- API precedenti (`emit`, `emit_and_count`, `emit_sync_sequential`) invariate.
- Alias `emit_wait` marcato `#[deprecated]` ma funzionante (finestra di deprecazione: non verrà rimosso prima di Milestone 3).
- Nessuna rottura per handler esistenti.

## Sicurezza & Robustezza
- Panics isolati (default) impediscono crash globali.
- `catch_panics` disattivabile per scenari high‑perf controllati.
- Nessun `unwrap` critico nei percorsi handler.

## Osservabilità
- Hook metrics personalizzabile (sink).
- Tracing opzionale (feature `event-tracing`) con overhead nullo se disattivato.
- `EmitReport` utilizzabile per logging / alerting.

## Esempio Rapido
```rust
let bus = EventBus::builder()
    .catch_panics(true)
    .metrics_sink(Box::new(MyMetricsSink::default()))
    .build();

bus.subscribe_result("app.user.*", |evt: &Event| {
    // handler logic
    Ok(())
})?;

let report = bus.emit_and_report("app.user.login", EventPayload::empty());
if report.panics > 0 {
    tracing::warn!(?report, "handler panicked");
}
```

## Overhead & Performance (preliminary)
- Panic isolation (`catch_unwind`): ~1–2% overhead per handler (path "no panic", microbenchmark locale 10k emit, 5 handler). Misura preliminare.
- Metrics sink noop: costo trascurabile (branch predicibile). Sink custom dipende dal lavoro svolto (consigliato batching se costoso).
- Tracing disattivato: zero chiamate log (feature gated).
- Emissione vs M0: nessun peggioramento percepibile in test interni (<2% differenza media, entro rumore statistico).

## Testing
- Copertura: reporting, error path, wildcard segment singolo e multi (`**`), root `**`, sequenzialità, macro.
- Pianificato in M2: property testing parità vecchio/nuovo matcher, benchmark criterione.

## Migrazione
Nessuna azione obbligatoria. Facoltativo adottare `emit_and_report` dove serve logica condizionata sugli esiti o integrazione metrics.

## Changelog (Milestone 1)
Added: `EventBusBuilder`, `subscribe_result`, `emit_and_report`, `HandlerResult`, `EmitReport`, `MetricsSink`, `NoopMetricsSink`, macro `event_emit!`
Improved: Panic handling, wildcard test coverage, README Event Bus
Deprecated: `emit_wait` (sostituito da `emit_and_count`)
Security: Panic isolation di default

## Roadmap (Estratto Prossime Fasi)
- M2: Pattern precompilation + benchmark suite, typed event bus design, property tests matcher, perf docs, pattern validation estesa (#15).
- M3: Backpressure, priority handlers, replay buffer, federazione multi-bus, interceptors, cancellazione cooperativa.

## Checklist Revisione
- [ ] `cargo build --all --release` ok
- [ ] `cargo test --all` (senza feature) ok
- [ ] `cargo test -p zenterm-core --features "event-tracing"` ok
- [ ] `cargo test -p zenterm-core --features "event-tracing,custom-payload"` ok (se applicabile)
- [ ] Nessun panic attraversa il bus con `catch_panics=true`
- [ ] Panic riprodotto correttamente (process crash) con `catch_panics=false`
- [ ] README / docs aggiornati (`emit_and_report` + esempio MetricsSink)
- [ ] Macro `event_emit!` testata in un modulo reale
- [ ] Deprecation `emit_wait` visibile (warning compilazione)
- [ ] Decisione mantenimento alias `emit_wait` fino a fine M2 (default: sì)

## Note per Release
Suggerito bump versione minor (x.(y+1).0) per nuove API additive. Attendere almeno 1 review e checklist completa prima del tag.