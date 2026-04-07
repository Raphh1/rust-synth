# Diagrammes C4 — MASE

## Niveau 1 — Context

```
┌─────────────────────────────────────────────────────────────┐
│                        Utilisateur                          │
│                    (musicien / compositeur)                  │
└─────────────────────┬───────────────────────────────────────┘
                      │ interagit avec
                      ▼
┌─────────────────────────────────────────────────────────────┐
│                         MASE                                │
│          Modular Audio Synthesis Engine                     │
│                                                             │
│  Synthétiseur logiciel permettant de composer et jouer      │
│  des séquences musicales avec un moteur DSP temps réel.     │
└─────────────────────────────────────────────────────────────┘
                      │ émet vers
                      ▼
              [ Carte son / OS audio ]
```

---

## Niveau 2 — Container

```
┌──────────────────────────────────────────────────────────────┐
│                          MASE                                │
│                                                              │
│  ┌─────────────────────────────┐                            │
│  │   UI — Avalonia C# (.NET 8) │   ← interaction utilisateur│
│  │                             │     rendu visuel           │
│  │  MainWindow, PianoRoll,     │     pas de logique métier  │
│  │  WaveformControl, Envelope  │                            │
│  └──────────────┬──────────────┘                            │
│                 │ IPC JSON stdin/stdout                      │
│                 │ (NDJSON, 1 message par ligne)              │
│                 ▼                                            │
│  ┌─────────────────────────────┐                            │
│  │   Backend — Rust            │   ← domaine métier         │
│  │                             │     validation             │
│  │  Domain (DDD)               │     séquenceur             │
│  │  Application (Engine)       │     moteur DSP temps réel  │
│  │  IPC (Dispatcher)           │                            │
│  └──────────────┬──────────────┘                            │
│                 │ cpal (WASAPI / ALSA)                       │
│                 ▼                                            │
│        [ Périphérique audio OS ]                            │
└──────────────────────────────────────────────────────────────┘
```

---

## Niveau 3 — Component (Backend Rust)

```
┌──────────────────────────────────────────────────────────────────┐
│                        Backend Rust                              │
│                                                                  │
│  ┌──────────────┐    ┌────────────────────────────────────────┐ │
│  │  ipc/        │    │  domain/                               │ │
│  │              │    │                                        │ │
│  │  protocol.rs │    │  patch/                                │ │
│  │  (Command,   │    │    model.rs  ← Aggregate Patch         │ │
│  │   Response)  │    │              ← Builder Pattern         │ │
│  │              │    │    validator.rs ← Domain Service       │ │
│  │  dispatcher  │───▶│                                        │ │
│  │  .rs         │    │  sequencing/                           │ │
│  │  (Command    │    │    pattern.rs ← Aggregate Pattern      │ │
│  │   Pattern)   │    │    transport.rs ← State Pattern        │ │
│  └──────┬───────┘    │                                        │ │
│         │            │  synthesis/                            │ │
│         │            │    dsp.rs    ← Composite Pattern       │ │
│         │            │              (DspNode, DspGraph)       │ │
│         │            │    wavetable.rs ← Value Object         │ │
│         │            │    filter.rs, envelope.rs,             │ │
│         │            │    lfo.rs, portamento.rs               │ │
│         │            └───────────────────┬────────────────────┘ │
│         │                                │                      │
│         ▼                                ▼                      │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  application/engine.rs                                   │   │
│  │  Service applicatif :                                    │   │
│  │  - Boucle IPC (stdin → dispatch → stdout)                │   │
│  │  - Thread audio (cpal callback → DSP pipeline)           │   │
│  │  - Arc<Mutex<EngineState>> partagé entre les deux        │   │
│  └──────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Niveau 4 — Code : pipeline DSP (par buffer audio)

```
EngineState (Mutex snapshot)
        │
        ▼
  seq_pos → Pattern.active_notes_at(seq_pos)
        │
        ▼
  gate signal (0.0 / 1.0)
        │
  ┌─────┴──────────────────────────────────────┐
  │                                             │
  ▼                                             ▼
OscillatorNode.process()              LFO.tick()
  (Wavetable lookup + interpolation)    (Sine/Square/Tri/Saw)
        │                                       │
        │                              lfo_cutoff_mod
        │                              lfo_pitch_mod
        │                              lfo_vol_mod
        ▼                                       │
  Portamento.tick() → fréquence modulée ────────┘
        │
        ▼
  EnvelopeNode.process(gate) → ADSR amplitude
        │
        ▼
  osc_out × env_out
        │
        ▼
  FilterNode.process() → biquad low-pass
        │
        ▼
  volume × lfo_vol_mod
        │
        ▼
  f32 sample → buffer cpal → carte son
```
