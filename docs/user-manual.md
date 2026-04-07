# Manuel Utilisateur — MASE

MASE (Modular Audio Synthesis Engine) est un synthétiseur logiciel monophonique avec séquenceur intégré.

---

## Lancer l'application

```bash
cd UI-rusty-synth/Mase.Ui
dotnet run
```

La fenêtre principale s'ouvre. Le moteur audio démarre automatiquement.

---

## Interface principale

### Barre de transport (haut)

| Élément | Rôle |
|---------|------|
| **Play ▶** | Démarre la lecture du pattern |
| **Stop ⏹** | Arrête la lecture |
| **Loop ↩** | Active/désactive la boucle |
| **BPM** | Tempo (40–250 BPM) — modifiable via le slider, les boutons +/- ou en tapant directement |
| **Open Piano Roll** | Ouvre le séquenceur dans une fenêtre dédiée |
| **Engine** | Indicateur d'état (vert = en lecture, rouge = arrêté) |

---

## Paramètres sonores (panneau gauche)

### Portamento
Glisse de fréquence entre les notes. À 0 = saut instantané. Augmenter pour un effet legato.

### Filtre
- **Cutoff** : fréquence de coupure du filtre passe-bas (20 Hz – 20 kHz)
- **Resonance** : accentuation autour de la fréquence de coupure (0 = neutre, 1 = très résonant)

### Envelope ADSR
- **A (Attack)** : temps de montée du son (0.001s – 2s)
- **D (Decay)** : temps de descente vers le sustain (0.001s – 2s)
- **S (Sustain)** : niveau maintenu pendant la note (0 – 1)
- **R (Release)** : temps d'extinction après relâchement (0.001s – 5s)

### LFO (Low Frequency Oscillator)
Modulation périodique d'un paramètre.

- **Shape** : forme d'onde du LFO (Sine, Square, Triangle, Saw)
- **Target** : paramètre modulé (Cutoff, Pitch, Volume)
- **Rate** : vitesse du LFO (0.1 – 20 Hz)
- **Depth** : intensité de la modulation (0 – 1)

---

## Waveform (panneau droite)

Affiche et permet de modifier la forme d'onde de l'oscillateur.

### Presets
Cliquez sur **Sine ~**, **Square ⊓**, **Saw /**, **Tri △** pour charger une forme prédéfinie.  
Ces boutons sont désactivés pendant la lecture.

### Mode édition
1. Cliquez sur **Edit** pour activer le mode dessin (bordure bleue)
2. Dessinez la forme d'onde à la souris
3. Le son change en temps réel à la prochaine lecture
4. **Reset** recharge la forme sinusoïdale

---

## Piano Roll

### Ouverture
Cliquez sur **Open Piano Roll** dans la barre de transport.

### Ajouter une note
Cliquez dans la grille à l'intersection de la note (axe vertical = hauteur, axe horizontal = temps).

### Supprimer une note
Clic droit sur une note existante.

### Déplacer une note
Glisser-déposer la note vers une nouvelle position.

### Redimensionner une note
Tirer le bord droit de la note pour modifier sa durée.

---

## Workflow typique

1. Ouvrir le Piano Roll → placer des notes
2. Régler le BPM
3. Ajuster l'enveloppe ADSR selon l'effet voulu
4. Choisir une forme d'onde (preset ou dessinée)
5. Activer Loop si nécessaire
6. Cliquer **Play ▶**
7. Ajuster le filtre et le LFO en temps réel pendant la lecture
