# Documentation du domaine

Comment les skills d'ingénierie doivent exploiter la documentation du domaine de ce repo lorsqu'ils explorent la base de code.

## Avant d'explorer, lis ceci

- **`CONTEXT.md`** à la racine du repo, ou
- **`CONTEXT-MAP.md`** à la racine du repo s'il existe : il pointe vers un `CONTEXT.md` par contexte. Lis chacun de ceux qui concernent le sujet.
- **`docs/adr/`** : lis les ADR qui touchent la zone dans laquelle tu vas travailler. Dans les repos multi-contextes, consulte aussi `src/<context>/docs/adr/` pour les décisions propres à un contexte.

Si l'un de ces fichiers n'existe pas, **continue sans rien dire**. Ne signale pas son absence et ne propose pas de le créer d'avance. Le skill `/domain-modeling` (atteint via `/grill-with-docs` et `/improve-codebase-architecture`) les crée au fil de l'eau, quand des termes ou des décisions sont réellement tranchés.

## Structure des fichiers

Repo mono-contexte (la plupart des repos) :

```
/
├── CONTEXT.md
├── docs/adr/
│   ├── 0001-event-sourced-orders.md
│   └── 0002-postgres-for-write-model.md
└── src/
```

Repo multi-contextes (présence de `CONTEXT-MAP.md` à la racine) :

```
/
├── CONTEXT-MAP.md
├── docs/adr/                          ← décisions à l'échelle du système
└── src/
    ├── ordering/
    │   ├── CONTEXT.md
    │   └── docs/adr/                  ← décisions propres au contexte
    └── billing/
        ├── CONTEXT.md
        └── docs/adr/
```

## Utilise le vocabulaire du glossaire

Quand ce que tu produis nomme un concept du domaine (dans un titre d'issue, une proposition de refactoring, une hypothèse, un nom de test), emploie le terme tel que défini dans `CONTEXT.md`. Ne dérive pas vers les synonymes que le glossaire demande explicitement d'éviter.

Si le concept dont tu as besoin n'est pas encore dans le glossaire, c'est un signal : soit tu inventes un vocabulaire que le projet n'emploie pas (reconsidère), soit il y a un vrai manque (note-le pour `/domain-modeling`).

## Signale les conflits avec les ADR

Si ce que tu produis contredit un ADR existant, signale-le explicitement au lieu de passer outre en silence :

> _Contredit l'ADR-0007 (commandes en event sourcing), mais mérite d'être rouvert parce que…_
