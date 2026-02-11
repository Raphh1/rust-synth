# Modélisation DDD – Modular Audio Synthesis Engine (MASE)

## 1. Approche Domain-Driven Design

Le système **MASE (Modular Audio Synthesis Engine)** est modélisé selon une approche **Domain-Driven Design (DDD)** afin de structurer clairement la logique métier, séparer les responsabilités et garantir la cohérence des règles du domaine audio temps réel.

Cette modélisation repose sur :

- un **langage ubiquitaire** partagé entre développeurs et utilisateurs,
- une décomposition en **Bounded Contexts**,
- des **agrégats métier clairement identifiés**,
- des **services de domaine isolant les règles critiques**.

---

## 2. Langage ubiquitaire (Ubiquitous Language)

Le domaine utilise les concepts suivants :

- **Patch** : configuration modulaire du son,
- **Module** : unité DSP (Oscillator, Filter, Envelope, etc.),
- **Connexion** : lien entre ports de modules,
- **Paramètre** : valeur modifiable influençant le son,
- **Validation** : vérification de la cohérence sonore,
- **Compilation** : transformation du patch en programme DSP,
- **Runtime DSP** : moteur audio temps réel,
- **Pattern** : séquence de notes,
- **NoteEvent** : événement musical,
- **Preset** : sauvegarde d’un état du système,
- **ExportJob** : rendu audio hors ligne,
- **WaveTable** : forme d’onde personnalisée.

Ces termes sont utilisés uniformément dans l’ensemble du système.

---

## 3. Bounded Contexts

Le système est découpé en plusieurs contextes métiers indépendants :

### BC1 – Patch Design  
Responsable de la construction du patch (modules, connexions, paramètres).

### BC2 – Patch Validation  
Garantit que le patch respecte les contraintes sonores et physiques.

### BC3 – Patch Compilation  
Transforme un patch valide en **programme DSP déterministe**.

### BC4 – Real-Time Synthesis  
Exécute le programme DSP en respectant les contraintes temps réel.

### BC5 – Sequencing  
Gère le transport, le tempo et la planification des notes.

### BC6 – Library and Export  
Gère la persistance et le rendu hors ligne.

### BC7 – Control and IPC  
Assure la communication entre l’IHM C# et le cœur Rust.

Cette séparation permet une isolation stricte des responsabilités.

---

## 4. Agrégats métier

### Agrégat Patch (racine d’agrégat)

Le **Patch** représente la structure sonore avant compilation.

Il contient :

- des **Modules** (entités),
- des **Connexions** (entités),
- des **Paramètres** (Value Objects).

**Invariants principaux :**

- connexions valides,
- paramètres bornés,
- structure cohérente.

---

### Service de domaine – PatchValidator

Le **PatchValidator** encapsule les règles métier critiques :

- absence de cycles audio,
- présence d’une sortie unique,
- compatibilité des ports,
- existence d’une source sonore.

Un patch invalide ne peut être compilé.

---

### Service de domaine – PatchCompiler

Le **PatchCompiler** transforme un patch valide en **DspProgram**, garantissant :

- ordre d’exécution déterministe,
- absence d’ambiguïté dans le graphe,
- exécution temps réel sûre.

---

### Agrégat Pattern

Le **Pattern** modélise la séquence musicale :

- NoteEvent (pitch, start, length),
- Tempo,
- TransportState.

**Invariants :**

- notes bornées,
- durée positive,
- position valide dans la grille.

---

### Agrégats Preset et ExportJob

- **Preset** : snapshot métier,
- **ExportJob** : rendu hors ligne contrôlé.

---

## 5. Domain Events

Le domaine expose des événements métier tels que :

- PatchValidated,
- PatchCompiled,
- ParameterChanged,
- NoteAdded,
- TransportStarted,
- ExportCompleted.

Ces événements permettent un découplage fort entre contextes.

---

## 6. Intégration UI – Backend

L’IHM Avalonia C# agit comme un **client externe**.

Elle :

- capture les interactions utilisateur,
- envoie des **commandes métier via IPC**,
- ne contient aucune logique métier audio.

Le backend Rust conserve l’intégralité des règles du domaine.

---

## 7. Principes architecturaux respectés

Cette modélisation garantit :

- indépendance du domaine vis-à-vis de l’IHM,
- isolation des règles critiques,
- conformité aux contraintes temps réel,
- forte testabilité du cœur métier.

---

## 8. Conclusion

L’application du Domain-Driven Design permet de structurer MASE comme un système cohérent, modulaire et extensible, tout en respectant les contraintes spécifiques de la synthèse audio temps réel.
