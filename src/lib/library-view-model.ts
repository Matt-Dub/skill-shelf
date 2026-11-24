/** Library view model: decides what the panel shows for each skill. Pure TypeScript. */

export type Skill = { name: string; description: string; path: string };

export type CardAction = {
  kind: "open-folder" | "open-skill-md";
  label: string;
  target: string;
};

export function cardActions(skill: Skill): CardAction[] {
  const separator = skill.path.includes("\\") ? "\\" : "/";
  return [
    { kind: "open-folder", label: "Open folder", target: skill.path },
    { kind: "open-skill-md", label: "Open SKILL.md", target: `${skill.path}${separator}SKILL.md` },
  ];
}
