# Skill Shelf

Un widget de bureau qui montre à l'utilisateur les skills installés sur sa machine, explique brièvement chacun d'eux et l'emmène vers leur provenance.

Les termes du glossaire restent en anglais : ce sont ceux qu'emploient le code, les issues et les noms de tests.

## Vocabulaire

### Les skills et leur provenance

**Skill** :
Un dossier contenant un `SKILL.md` qu'un agent peut charger ; identifié par son nom.
_À éviter_ : Command, prompt, tool

**Library** :
L'ensemble des skills dont dispose l'utilisateur : ses user skills, les skills apportés par ses plugins et les skills intégrés à l'agent. Les skills de projet n'en font pas partie.
_À éviter_ : Collection, catalog, inventory

**User skill** :
Un skill installé au niveau utilisateur, qu'il se trouve directement dans le dossier de skills de l'agent ou qu'il y soit lié depuis un shared skills store.

**Plugin skill** :
Un skill livré dans un plugin installé ; il appartient à ce plugin. Quand le plugin est installé mais pas activé, ses skills font toujours partie de la Library mais sont **désactivés**.

**Built-in skill** :
Un skill livré avec l'agent lui-même plutôt qu'installé par l'utilisateur ; il n'a pas de dossier lisible sur le disque.
_À éviter_ : Native skill, bundled skill

**Origin** :
La provenance d'un skill, vers laquelle le widget envoie l'utilisateur pour en savoir plus : le dépôt distant depuis lequel il a été installé ou, pour un built-in skill, la documentation ou le dépôt de l'éditeur de l'agent. Un skill écrit localement par l'utilisateur n'a pas d'origin.
_À éviter_ : Source, upstream

**Agent** :
Un assistant de code dont le widget lit les skills (Claude Code d'abord ; d'autres plus tard).

### Ce que montre le widget

**Description** :
Le résumé d'une ligne du skill, tel que rédigé par son auteur.

**Use case** :
Une courte indication du moment où utiliser le skill, tirée d'un champ explicite si l'auteur en a fourni un, sinon extraite de la description.
_À éviter_ : Trigger, example

**Pinned mode** :
L'état du widget lorsqu'il est posé sur le bureau, sous les autres fenêtres, au lieu de s'ouvrir depuis la zone de notification.
