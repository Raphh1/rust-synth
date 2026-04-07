# Manuel Technique — MASE (Modular Audio Synthesis Engine)

## Stack technique

| Composant | Technologie |
|-----------|-------------|
| Backend (domaine + audio) | Rust 2024 edition |
| Frontend (IHM) | C# .NET 8, Avalonia UI, CommunityToolkit.Mvvm |
| Communication inter-processus | JSON over stdin/stdout (NDJSON) |
| Audio | cpal 0.15 (Cross-Platform Audio Library) |
| Sérialisation | serde + serde_json |

---

## Architecture

Le projet suit une architecture **Domain-Driven Design (DDD)** avec séparation stricte en couches :

```
UI C# (Avalonia)          ← présentation uniquement
       ↕ IPC JSON (stdin/stdout)
Backend Rust
  ├── ipc/               ← protocole + dispatcher (Command Pattern)
  ├── application/       ← engine (service applicatif, audio cpal)
  └── domain/
       ├── patch/        ← Aggregate Patch, PatchBuilder, PatchValidator
       ├── sequencing/   ← Aggregate Pattern, Transport (State Pattern)
       └── synthesis/    ← Wavetable (VO), DspGraph (Composite Pattern),
                            OscillatorNode, FilterNode, EnvelopeNode, Lfo, Portamento
```

### Design Patterns implémentés

| Pattern | Type | Fichier |
|---------|------|---------|
| **Builder** | Création | `domain/patch/model.rs` — `PatchBuilder` |
| **Composite** | Structurel | `domain/synthesis/dsp.rs` — `DspNode` trait + `DspGraph` |
| **State** | Comportemental | `domain/sequencing/transport.rs` — `TransportState` |
| **Command** | Comportemental | `ipc/dispatcher.rs` — `dispatch()` |

---

## Structure des dossiers

```
rust-synth/
├── backend/                   # Crate Rust
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── application/
│       │   └── engine.rs      # Service applicatif + boucle audio cpal
│       ├── domain/
│       │   ├── patch/
│       │   │   ├── model.rs   # Aggregate Patch + Builder Pattern
│       │   │   └── validator.rs
│       │   ├── sequencing/
│       │   │   ├── pattern.rs # Aggregate Pattern (NoteEvent)
│       │   │   └── transport.rs # State Pattern (Play/Stop)
│       │   └── synthesis/
│       │       ├── dsp.rs     # Composite Pattern (DspNode, DspGraph)
│       │       ├── envelope.rs
│       │       ├── filter.rs  # Biquad low-pass (RBJ EQ Cookbook)
│       │       ├── lfo.rs
│       │       ├── portamento.rs
│       │       └── wavetable.rs # Value Object
│       └── ipc/
│           ├── dispatcher.rs  # Command Pattern
│           ├── protocol.rs    # Types serde (Command, Response)
│           └── server.rs
├── frontend/                  # Projet Avalonia C#
│   ├── Mase.Ui.csproj
│   ├── Services/
│   │   └── IpcService.cs      # Spawn + communication avec le backend
│   ├── ViewModels/
│   │   └── MainViewModel.cs
│   ├── Views/
│   ├── Controls/
│   │   ├── PianoRollGrid.cs
│   │   ├── WaveformControl.cs
│   │   └── EnvelopeControl.cs
│   └── Models/
│       └── NoteModel.cs
├── docs/
└── rust-synth.sln
```

---

## Prérequis

- **Rust** ≥ 1.80 (edition 2024) : [rustup.rs](https://rustup.rs)
- **.NET 8 SDK** : [dotnet.microsoft.com](https://dotnet.microsoft.com)
- **Windows** : périphérique audio WASAPI (inclus par défaut)
- **Linux** : ALSA ou PulseAudio

---

## Build & Run

### Backend seul

```bash
cd backend
cargo build --release
cargo test
```

### Application complète

```bash
cd frontend
dotnet run
```

L'UI spawn automatiquement le backend (`backend/target/debug/backend.exe`). Ne pas lancer `cargo run` séparément.

---

## Protocole IPC

Communication JSON one-line (NDJSON) sur stdin/stdout du processus backend.

Chaque commande contient un `request_id` ; chaque réponse retourne le même `request_id`.

**Commandes disponibles :** `play`, `stop`, `setParam`, `addNote`, `moveNote`, `resizeNote`, `deleteNote`, `wavetableSet`, `loopToggle`, `patchReplace`, `patchValidate`

**Réponses :** `{"type":"ok","request_id":"..."}` ou `{"type":"error","request_id":"...","code":"...","message":"..."}`

Voir [ipc_protocol.md](../ipc_protocol.md) pour le contrat complet.

---

## Tests

```bash
cd backend
cargo test
```

65 tests couvrant : dispatcher IPC, Pattern aggregate, Transport, Validator, Wavetable, Envelope, Oscillateur, Filtre, LFO, Portamento.

---

## Dépendances

```toml
[dependencies]
serde       = { version = "1", features = ["derive"] }
serde_json  = "1.0"
cpal        = "0.15"
```

Frontend C# : `Avalonia`, `CommunityToolkit.Mvvm` (via NuGet).
