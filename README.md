# MASE — Modular Audio Synthesis Engine

Synthétiseur logiciel monophonique avec séquenceur intégré.  
Backend **Rust** (domaine + audio temps réel) + Frontend **C# Avalonia** (IHM).

---

## Démarrage rapide

```bash
cd frontend
dotnet run
```

L'UI démarre et lance automatiquement le backend Rust.

---

## Architecture

Approche **Domain-Driven Design** avec communication inter-processus JSON (NDJSON stdin/stdout).

```
UI C# (Avalonia)
    ↕ IPC JSON
Backend Rust
  ├── ipc/         Command Pattern — dispatch des commandes
  ├── application/ Engine applicatif + thread audio (cpal)
  └── domain/
       ├── patch/        Aggregate Patch + Builder Pattern + PatchValidator
       ├── sequencing/   Aggregate Pattern + Transport (State Pattern)
       └── synthesis/    Wavetable (VO) + DspGraph (Composite Pattern)
                         Oscillator, Filter, Envelope, LFO, Portamento
```

**Design patterns :** Builder (Patch), Composite (DspGraph), State (Transport), Command (Dispatcher)

---

## Tests

```bash
cd backend
cargo test
```

65 tests — couvrent : dispatcher IPC, Pattern, Transport, Validator, Wavetable, Envelope, Oscillateur, Filtre, LFO, Portamento.

---

## Documentation

| Document | Contenu |
|----------|---------|
| [docs/functional-description.md](docs/functional-description.md) | Besoin métier, domaine, bounded contexts, règles |
| [docs/technical-manual.md](docs/technical-manual.md) | Architecture, build, IPC, design patterns, dépendances |
| [docs/user-manual.md](docs/user-manual.md) | Guide utilisateur complet |
| [docs/c4-diagram.md](docs/c4-diagram.md) | Diagrammes C4 niveaux 1 à 4 |
| [modelisation-DDD.md](modelisation-DDD.md) | Modélisation DDD détaillée |
| [ipc_protocol.md](ipc_protocol.md) | Contrat IPC complet |

---

## Stack

| Composant | Technologie |
|-----------|-------------|
| Backend | Rust 2024, cpal 0.15, serde_json |
| Frontend | C# .NET 8, Avalonia UI, CommunityToolkit.Mvvm |
| IPC | NDJSON (stdin/stdout) |
