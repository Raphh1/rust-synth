# IPC Contract - MASE (UI C# Avalonia <-> Backend Rust)

## Objectif
Ce document definit le contrat de communication entre :
- UI : application C# Avalonia (interaction utilisateur, affichage, edition)
- Backend : processus Rust (domaine, validation, sequencer, moteur audio)

Le contrat IPC garantit que les deux processus peuvent se comprendre sans ambiguite.

---

## Principe fondamental
- UI et Backend sont deux processus distincts
- Ils ne partagent ni memoire ni types
- Toute communication passe par des messages IPC structures

Sans contrat IPC -> erreurs d integration et bugs imprevisibles

---

## Format des messages
- Encodage : UTF-8
- Format : JSON
- Framing : NDJSON (1 message JSON par ligne)

Exemple :
{"type":"Play","requestId":"1"}
{"type":"SetParam","requestId":"2","path":"osc.freq","value":440.0}

---

## Regle critique : requestId
Chaque commande envoyee par l UI doit contenir :
- requestId : string unique

Le backend renvoie toujours une reponse avec le meme requestId.

But : associer requetes et reponses.

---

## Modele de communication
UI -> envoie une commande
Backend -> repond :

- Ok -> succes
- Error -> echec

---

## Reponses standard

### Ok
{"type":"Ok","requestId":"123"}

---

### Error
{
  "type":"Error",
  "requestId":"123",
  "code":"ParamOutOfRange",
  "message":"Value outside allowed range"
}

Codes d erreur MVP recommandes :
- InvalidMessage
- UnknownCommand
- ParamNotFound
- ParamOutOfRange
- PatchInvalid
- EditDeniedPlaying
- ExportFailed
- PresetNotFound
- InternalError

---

## Commandes minimales (MVP)

### Play
Demarre la lecture audio.
{"type":"Play","requestId":"1"}

---

### Stop
Arrete la lecture audio.
{"type":"Stop","requestId":"2"}

---

### SetParam
Modifie un parametre temps reel.
{
  "type":"SetParam",
  "requestId":"3",
  "path":"filter.cutoff",
  "value":1200.0
}

Regles :
- path inconnu -> ParamNotFound
- valeur hors bornes -> ParamOutOfRange

---

### AddNote
Ajoute une note au pattern.
{
  "type":"AddNote",
  "requestId":"4",
  "note":{
    "pitch":60,
    "start":0,
    "length":4,
    "velocity":0.9
  }
}

---

### MoveNote
Deplace une note existante.
{
  "type":"MoveNote",
  "requestId":"5",
  "id":"n1",
  "pitch":62,
  "start":4
}

---

### ResizeNote
Change la duree d une note.
{
  "type":"ResizeNote",
  "requestId":"6",
  "id":"n1",
  "length":8
}

---

### DeleteNote
Supprime une note.
{
  "type":"DeleteNote",
  "requestId":"7",
  "id":"n1"
}

---

### PatchReplace
Remplace le patch complet.
{
  "type":"PatchReplace",
  "requestId":"8",
  "patch":{
    "modules":[
      {"id":"osc1","kind":"Oscillator","params":{"wave":"sine","gain":0.8}},
      {"id":"out1","kind":"Output","params":{}}
    ],
    "connections":[
      {"from":"osc1.out","to":"out1.in"}
    ]
  }
}

Regles :
- patch invalide -> PatchInvalid

---

### PatchValidate
Demande validation du patch courant.
{"type":"PatchValidate","requestId":"9"}

---

### WavetableSet
Definit une waveform custom. Autorise uniquement en STOP.
{
  "type":"WavetableSet",
  "requestId":"10",
  "oscId":"osc1",
  "table":[0.0,0.1,0.0,-0.1]
}

Regles :
- si Playing -> EditDeniedPlaying
- table invalide -> InvalidMessage

---

## Philosophie d architecture
- UI : presentation, edition, visualisation uniquement
- Backend Rust : source de verite, validation, logique metier, audio
- IPC : frontiere stable entre systemes

---

## Version du protocole
protocolVersion : 1.0 (optionnel)

Exemple :
{"type":"Play","requestId":"1","protocolVersion":"1.0"}

---

## Resume ultra simple
Le contrat IPC est la grammaire officielle entre UI et Backend Rust.
Il definit exactement comment les deux applications se parlent.
