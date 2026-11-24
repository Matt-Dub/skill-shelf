/** Library view model: decides what the panel shows for each skill. Pure TypeScript. */

export type Origin = { url: string; repo: string };

export type Skill = { name: string; description: string; path: string; origin: Origin | null };

export type CardAction = {
  kind: "open-origin" | "open-folder" | "open-skill-md";
  label: string;
  target: string;
};

export function cardActions(skill: Skill): CardAction[] {
  const separator = skill.path.includes("\\") ? "\\" : "/";
  const local: CardAction[] = [
    { kind: "open-folder", label: "Open folder", target: skill.path },
    { kind: "open-skill-md", label: "Open SKILL.md", target: `${skill.path}${separator}SKILL.md` },
  ];
  if (!skill.origin) return local;
  return [{ kind: "open-origin", label: "Open Origin", target: skill.origin.url }, ...local];
}

export type SkillGroup = { title: string; skills: Skill[] };

export const LOCAL_SKILLS = "Local skills";

/** Groups the Library by Origin repository (case-insensitive order); skills without an Origin come last. */
export function groupByOrigin(skills: Skill[]): SkillGroup[] {
  const byRepo = new Map<string, Skill[]>();
  const local: Skill[] = [];
  for (const skill of skills) {
    if (!skill.origin) {
      local.push(skill);
      continue;
    }
    const group = byRepo.get(skill.origin.repo) ?? [];
    group.push(skill);
    byRepo.set(skill.origin.repo, group);
  }
  const groups = [...byRepo]
    .sort(([a], [b]) => a.localeCompare(b, undefined, { sensitivity: "base" }))
    .map(([title, skills]) => ({ title, skills }));
  return local.length ? [...groups, { title: LOCAL_SKILLS, skills: local }] : groups;
}
