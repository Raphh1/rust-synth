# Plan : Finalisation du backend MASE

## Contexte
Projet scolaire d'architecture logiciel — synthétiseur modulaire (MASE). Backend Rust + Frontend C# Avalonia (IPC JSON). L'audio doit réellement fonctionner. Deadline : 7 jours, équipe de 2.

---

## Phase 1 — Modèles domaine (Jour 1-2)

### 1. `domain/patch/model.rs` — Aggregate Patch + **Builder Pattern** (création)
- **Value Objects** : `ParamValue` (name, value, min, max), `PortId` (module_id, port_name)
- **Entités** : `Module` (id, kind, params), `Connection` (from: PortId, to: PortId)
- **Aggregate root** : `Patch` (modules, connections)
- **PatchBuilder** : `new()` → `add_module()` → `add_connection()` → `build() -> Result<Patch>`
- `Patch::set_param(path, value)` parse "module.param", vérifie bornes
- `PatchError` enum typé
- Conversion `ipc::protocol::Patch → domain::Patch` (anti-corruption layer)

### 2. `domain/patch/validator.rs` — Service PatchValidator
- `PatchValidator::validate(&Patch) -> Result<(), Vec<String>>`
- Règles : ≥1 Output, ≥1 Oscillator, connexions valides, pas de cycles (algo de Kahn)

### 3. `domain/sequencing/pattern.rs` — Aggregate Pattern
- `Pattern` contient `Vec<NoteEvent>` + compteur d'ID
- Méthodes : `add_note()`, `move_note()`, `resize_note()`, `delete_note()`, `active_notes_at(position)`
- Invariants : length > 0, velocity ∈ [0,1], pitch ∈ [0,127]

### 4. `domain/sequencing/transport.rs` — **State Pattern** (comportemental)
- Rendre `TransportState` public
- Déjà implémenté (play/stop avec transitions d'état)

### 5. `domain/synthesis/wavetable.rs` — Value Object Wavetable
- `Wavetable::sine(size)`, `saw(size)`, `square(size)`
- `sample_at(phase) -> f64` avec interpolation linéaire

### 6. `domain/synthesis/dsp.rs` (nouveau) — **Composite Pattern** (structurel)
- Trait `DspNode` : `process()`, `set_param()`, `note_on()`, `note_off()`
- Nœuds feuilles : `OscillatorNode`, `FilterNode`, `EnvelopeNode`, `OutputNode`, `MixerNode`
- `DspGraph` (composite) : nodes ordonnés par tri topologique, `process_sample() -> f64`

### 7. `domain/synthesis/compiler.rs` (nouveau) — Service PatchCompiler
- `PatchCompiler::compile(&Patch) -> Result<DspGraph>` : tri topo → instanciation des DspNode

---

## Phase 2 — Couche application + Audio (Jour 3-4)

### 8. `application/engine.rs` — Service applicatif Engine
- Possède : `Patch`, `Pattern`, `TransportState`, `Arc<Mutex<DspGraph>>`, `cpal::Stream`
- Méthodes : `play()`, `stop()`, `set_param()`, `add_note()`, `move_note()`, `resize_note()`, `delete_note()`, `replace_patch()`, `validate_patch()`, `set_wavetable()`
- `play()` : valide → compile → ouvre stream cpal → callback audio

### 9. `application/audio.rs` (nouveau) — Intégration cpal
- `start_audio_stream(dsp_graph, pattern, position, bpm) -> Result<(Stream, sample_rate)>`
- Callback : lock DspGraph, lire position, note_on/off, process_sample, écrire buffer

---

## Phase 3 — Couche IPC (Jour 4-5)

### 10. `ipc/dispatcher.rs` — **Command Pattern** (comportemental)
- `Dispatcher` possède un `Engine`
- `dispatch(Command) -> Response` : match sur les 10 commandes, appelle Engine, mappe erreurs

### 11. Refactorer `ipc/server.rs`
- Simplifier : boucle stdin → `dispatcher.dispatch()` → stdout
- Supprimer `IpcServer` et `TransportState` dupliqué

---

## Phase 4 — Tests ≥30% couverture (Jour 5-6)

| Fichier | Tests (~36 total) |
|---------|-------------------|
| `patch/model.rs` | 8 tests (builder, set_param, find_module) |
| `patch/validator.rs` | 6 tests (cycles, output, source, connexions) |
| `sequencing/pattern.rs` | 6 tests (CRUD notes, active_notes_at) |
| `sequencing/transport.rs` | 4 tests (transitions d'état) |
| `synthesis/wavetable.rs` | 4 tests (génération, interpolation) |
| `synthesis/dsp.rs` | 4 tests (oscillator, filter, graph) |
| `ipc/dispatcher.rs` | 4 tests (dispatch commandes) |

---

## Phase 5 — Documentation (Jour 7)

- **Diagrammes C4** : Context, Container, Component, Code
- **Manuel technique** : architecture, langages, dépendances, comment build/run
- **Manuel utilisateur** : description fonctionnelle, utilisation du synthé
- **Description technique** : Rust, cpal, serde, architecture DDD

---

## Dépendances Cargo.toml

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
cpal = "0.15"
```

---

## Design Patterns pour la notation

| Pattern | Type | Fichier |
|---------|------|---------|
| **Builder** | Création | `patch/model.rs` — `PatchBuilder` |
| **Composite** | Structurel | `synthesis/dsp.rs` — `DspNode` + `DspGraph` |
| **State** | Comportemental | `sequencing/transport.rs` — `TransportState` |
| **Command** | Comportemental (bonus) | `ipc/dispatcher.rs` — `Dispatcher` |

---

## Mapping DDD pour la notation

| Concept DDD | Implémentation |
|-------------|---------------|
| **BC: Patch Design** | `domain/patch/model.rs` |
| **BC: Patch Validation** | `domain/patch/validator.rs` |
| **BC: Patch Compilation** | `domain/synthesis/compiler.rs` |
| **BC: Real-Time Synthesis** | `domain/synthesis/dsp.rs` + `application/audio.rs` |
| **BC: Sequencing** | `domain/sequencing/` |
| **BC: Control/IPC** | `ipc/` |
| **Aggregate: Patch** | `Patch` (root) → `Module` (entité), `Connection` (entité), `ParamValue` (VO) |
| **Aggregate: Pattern** | `Pattern` (root) → `NoteEvent` (entité) |
| **Value Objects** | `ParamValue`, `PortId`, `Wavetable` |
| **Domain Services** | `PatchValidator`, `PatchCompiler` |
| **Application Service** | `Engine` |
| **Anti-corruption layer** | `protocol.rs` ↔ domain types, conversion dans `dispatcher.rs` |

---

## Répartition du travail (2 personnes)

- **Personne A** (J1-3) : Tout le domain layer + tests domaine
- **Personne B** (J1-3) : Spike audio cpal (jouer un sinus), squelette dispatcher
- **Ensemble** (J4-5) : Intégration engine + dispatcher + tests e2e
- **Ensemble** (J6-7) : Tests restants + documentation

---

## Risque principal
⚠️ **WSL2 n'a pas d'audio natif** — tester sur Windows natif ou configurer PulseAudio. Prévoir un flag `--no-audio` pour les tests.

---

## Vérification end-to-end
1. `cargo build` compile sans erreur
2. `cargo test` — tous les tests passent, couverture ≥30%
3. Lancer le binaire, envoyer des commandes JSON sur stdin, vérifier les réponses
4. Envoyer PatchReplace + Play → du son sort des enceintes
5. Envoyer AddNote + Play → les notes sont jouées au bon moment

---

## Arborescence finale

```
backend/src/
  main.rs
  ipc/
    mod.rs
    protocol.rs          (types IPC, ~80 lignes)
    server.rs            (boucle stdin/stdout, ~30 lignes)
    dispatcher.rs        (Command pattern, ~100 lignes)
  domain/
    mod.rs
    patch/
      mod.rs
      model.rs           (Patch aggregate + Builder, ~200 lignes)
      validator.rs       (PatchValidator service, ~80 lignes)
    sequencing/
      mod.rs
      transport.rs       (State pattern, ~35 lignes)
      pattern.rs         (Pattern aggregate, ~100 lignes)
    synthesis/
      mod.rs
      wavetable.rs       (Wavetable VO, ~60 lignes)
      dsp.rs             (Composite pattern, DspNode + DspGraph, ~250 lignes)
      compiler.rs        (PatchCompiler service, ~80 lignes)
  application/
    mod.rs
    engine.rs            (Engine service, ~200 lignes)
    audio.rs             (cpal intégration, ~80 lignes)
```

**Estimation totale** : ~1300 lignes de code + ~400 lignes de tests
