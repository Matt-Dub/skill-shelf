# Skill Shelf

A desktop widget that shows a user the skills installed on their machine, explains each one briefly, and takes them to where it comes from.

## Language

### Skills and where they come from

**Skill**:
A folder containing a `SKILL.md` that an agent can load; identified by its name.
_Avoid_: Command, prompt, tool

**Library**:
The full set of skills available to the user: their user skills, the skills brought by their plugins, and the agent's built-in skills. Project skills are not part of it.
_Avoid_: Collection, catalog, inventory

**User skill**:
A skill installed at user level, whether it lives directly in the agent's skills folder or is linked there from a shared skills store.

**Plugin skill**:
A skill shipped inside an installed plugin; it belongs to that plugin. When the plugin is installed but not enabled, its skills are still in the Library but are **disabled**.

**Built-in skill**:
A skill that ships with the agent itself rather than being installed by the user; it has no readable folder on disk.
_Avoid_: Native skill, bundled skill

**Origin**:
Where a skill comes from and where the widget sends the user to learn more: the remote repository it was installed from, or, for a built-in skill, the agent vendor's documentation or repository. A skill written locally by the user has no origin.
_Avoid_: Source, upstream

**Agent**:
A coding assistant whose skills the widget reads (Claude Code first; others later).

### What the widget shows

**Description**:
The skill's own one-line summary, as written by its author.

**Use case**:
A short statement of when to reach for the skill, taken from an explicit field if the author provided one, otherwise extracted from the description.
_Avoid_: Trigger, example

**Pinned mode**:
The widget's state when it sits on the desktop beneath other windows, instead of opening from the system tray.
