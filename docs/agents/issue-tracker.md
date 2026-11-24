# Issue tracker : GitHub

Les issues et les specs de ce repo sont des issues GitHub. Utilise la CLI `gh` pour toutes les opérations.

## Conventions

- **Créer une issue** : `gh issue create --title "..." --body "..."`. Utilise un heredoc pour les corps sur plusieurs lignes.
- **Lire une issue** : `gh issue view <number> --comments`, en filtrant les commentaires avec `jq` et en récupérant aussi les labels.
- **Lister les issues** : `gh issue list --state open --json number,title,body,labels,comments --jq '[.[] | {number, title, body, labels: [.labels[].name], comments: [.comments[].body]}]'` avec les filtres `--label` et `--state` appropriés.
- **Commenter une issue** : `gh issue comment <number> --body "..."`
- **Ajouter / retirer des labels** : `gh issue edit <number> --add-label "..."` / `--remove-label "..."`
- **Fermer** : `gh issue close <number> --comment "..."`

Déduis le repo de `git remote -v` ; `gh` le fait automatiquement lorsqu'il est lancé dans un clone.

## Les pull requests comme surface de triage

**PR comme surface de demandes : non.** _(Mettre à `oui` si ce repo traite les PR externes comme des demandes de fonctionnalité ; `/triage` lit ce paramètre.)_

Quand il vaut `oui`, les PR passent par les mêmes labels et états que les issues, avec les équivalents `gh pr` :

- **Lire une PR** : `gh pr view <number> --comments` et `gh pr diff <number>` pour le diff.
- **Lister les PR externes à trier** : `gh pr list --state open --json number,title,body,labels,author,authorAssociation,comments`, puis ne garder que les `authorAssociation` valant `CONTRIBUTOR`, `FIRST_TIME_CONTRIBUTOR` ou `NONE` (écarter `OWNER`/`MEMBER`/`COLLABORATOR`).
- **Commenter / labelliser / fermer** : `gh pr comment`, `gh pr edit --add-label`/`--remove-label`, `gh pr close`.

GitHub partage une même numérotation entre issues et PR, donc un `#42` seul peut désigner l'une ou l'autre : résous-le avec `gh pr view 42`, puis à défaut `gh issue view 42`.

## Quand un skill dit « publier dans l'issue tracker »

Crée une issue GitHub.

## Quand un skill dit « récupérer le ticket concerné »

Lance `gh issue view <number> --comments`.

## Opérations de wayfinding

Utilisées par `/wayfinder`. La **carte** est une issue unique dont les tickets sont des issues **enfants**.

- **Carte** : une issue unique portant le label `wayfinder:map`, dont le corps contient les sections Notes / Décisions jusqu'ici / Brouillard. `gh issue create --label wayfinder:map`.
- **Ticket enfant** : une issue liée à la carte comme sous-issue GitHub (`gh api` sur l'endpoint des sous-issues). Là où les sous-issues ne sont pas activées, ajoute l'enfant à une liste de tâches dans le corps de la carte et mets `Part of #<map>` en tête du corps de l'enfant. Labels : `wayfinder:<type>` (`research`/`prototype`/`grilling`/`task`). Une fois pris, le ticket est assigné au développeur qui le pilote.
- **Blocage** : les **dépendances natives entre issues** de GitHub, la représentation canonique et visible dans l'interface. Ajoute un lien avec `gh api --method POST repos/<owner>/<repo>/issues/<child>/dependencies/blocked_by -F issue_id=<blocker-db-id>`, où `<blocker-db-id>` est l'**identifiant de base de données** numérique du bloqueur (`gh api repos/<owner>/<repo>/issues/<n> --jq .id`, _pas_ le `#number` ni le `node_id`). GitHub renvoie `issue_dependencies_summary.blocked_by` (bloqueurs ouverts uniquement, la barrière effective). Là où les dépendances ne sont pas disponibles, utilise à la place une ligne `Blocked by: #<n>, #<n>` en tête du corps de l'enfant. Un ticket est débloqué quand tous ses bloqueurs sont fermés.
- **Requête de frontière** : liste les enfants ouverts de la carte (`gh issue list --state open`, limité aux sous-issues / à la liste de tâches de la carte), écarte ceux qui ont un bloqueur ouvert (`issue_dependencies_summary.blocked_by > 0`, ou une issue ouverte dans la ligne `Blocked by`) ou un assigné ; le premier dans l'ordre de la carte l'emporte.
- **Prendre un ticket** : `gh issue edit <n> --add-assignee @me`, première écriture de la session.
- **Résoudre** : `gh issue comment <n> --body "<answer>"`, puis `gh issue close <n>`, puis ajoute un pointeur de contexte (résumé + lien) aux Décisions jusqu'ici de la carte.
