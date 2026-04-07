# Description fonctionnelle — MASE

## Besoin métier

MASE répond au besoin de créer et jouer des séquences musicales avec un synthétiseur logiciel entièrement configurable, sans matériel externe.

L'utilisateur doit pouvoir :
- Composer une mélodie via un séquenceur (piano roll)
- Façonner le timbre sonore (forme d'onde, filtre, enveloppe)
- Animer le son en temps réel (LFO, portamento)
- Contrôler la lecture (play, stop, loop, tempo)

---

## Domaine métier

Le domaine est celui de la **synthèse audio logicielle**. Les concepts clés sont :

| Concept | Description |
|---------|-------------|
| **Patch** | Configuration modulaire du synthétiseur (oscillateurs, filtres, connexions) |
| **Wavetable** | Forme d'onde stockée sous forme d'échantillons numériques |
| **Pattern** | Séquence de NoteEvents définissant la mélodie |
| **NoteEvent** | Note musicale (pitch MIDI, position temporelle, durée, vélocité) |
| **Transport** | Gestion de l'état de lecture (Stopped / Playing) et du tempo (BPM) |
| **Envelope** | Courbe ADSR controlant l'amplitude dans le temps |
| **Filtre** | Transforme le spectre fréquentiel du signal (passe-bas biquad) |
| **LFO** | Oscillateur basse fréquence modulant un paramètre en temps réel |
| **Portamento** | Glissement de fréquence entre deux notes consécutives |

---

## Bounded Contexts

| Contexte | Responsabilité |
|----------|---------------|
| **Patch Design** | Construction et validation de la configuration modulaire |
| **Sequencing** | Gestion du pattern de notes et du transport |
| **Real-Time Synthesis** | Rendu audio échantillon par échantillon |
| **Control / IPC** | Communication entre l'IHM C# et le moteur Rust |

---

## Flux fonctionnel principal

```
Utilisateur
  → Place des notes dans le Piano Roll
  → Configure le timbre (waveform, filtre, envelope, LFO)
  → Clique Play
  
UI (C#)
  → Envoie AddNote (IPC JSON)
  → Envoie Play (IPC JSON)

Backend (Rust)
  → Valide et stocke les notes dans le Pattern aggregate
  → Démarre le Transport (état Playing)
  → Le thread audio lit le Pattern, génère le signal DSP
  → Applique oscillateur → enveloppe → filtre → sortie

Audio
  → Signal transmis à la carte son via cpal (WASAPI/ALSA)
```

---

## Règles métier principales

- Une note doit avoir : pitch ∈ [0, 127], durée > 0, vélocité ∈ [0, 1], start ≥ 0
- Le moteur est **monophonique** : une seule note active à la fois
- La waveform custom ne peut être modifiée qu'à l'arrêt
- Le Transport refuse un second `Play` si déjà en lecture (invariant d'état)
- Le filtre est borné : cutoff ∈ [20 Hz, 20 kHz], resonance ∈ [0, 1]
