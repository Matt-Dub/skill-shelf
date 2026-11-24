import { describe, expect, it } from "vitest";
import { cardActions, groupByOrigin, type Skill } from "./library-view-model";

const tdd: Skill = {
  name: "tdd",
  description: "Test-driven development.",
  path: "/home/ada/.claude/skills/tdd",
  origin: null,
};

describe("cardActions", () => {
  it("offers Open Origin as the main action, before the local actions", () => {
    const diagnose: Skill = {
      name: "diagnose",
      description: "Diagnosis loop.",
      path: "/home/ada/.claude/skills/diagnose",
      origin: {
        url: "https://github.com/mattpocock/skills/tree/HEAD/skills/engineering/diagnose",
        repo: "mattpocock/skills",
      },
    };
    expect(cardActions(diagnose).map((a) => [a.kind, a.target])).toEqual([
      ["open-origin", "https://github.com/mattpocock/skills/tree/HEAD/skills/engineering/diagnose"],
      ["open-folder", "/home/ada/.claude/skills/diagnose"],
      ["open-skill-md", "/home/ada/.claude/skills/diagnose/SKILL.md"],
    ]);
  });

  it("offers only the local actions for a skill without an Origin", () => {
    expect(cardActions(tdd)).toEqual([
      { kind: "open-folder", label: "Open folder", target: "/home/ada/.claude/skills/tdd" },
      { kind: "open-skill-md", label: "Open SKILL.md", target: "/home/ada/.claude/skills/tdd/SKILL.md" },
    ]);
  });

  it("points at SKILL.md with the separator of a Windows path", () => {
    const skill = { ...tdd, path: "C:\\Users\\ada\\.claude\\skills\\tdd" };
    const openSkillMd = cardActions(skill).find((a) => a.kind === "open-skill-md");
    expect(openSkillMd?.target).toBe("C:\\Users\\ada\\.claude\\skills\\tdd\\SKILL.md");
  });
});

describe("groupByOrigin", () => {
  const skill = (name: string, repo: string | null): Skill => ({
    name,
    description: `${name} description`,
    path: `/home/ada/.claude/skills/${name}`,
    origin: repo ? { url: `https://github.com/${repo}`, repo } : null,
  });

  it("groups skills by repository, sorted by repository, with local skills last", () => {
    const library = [
      skill("caveman", "JuliusBrussee/caveman"),
      skill("diagnose", "mattpocock/skills"),
      skill("mine", null),
      skill("copywriting", "coreyhaines31/marketingskills"),
      skill("tdd", "mattpocock/skills"),
      skill("zzz-notes", null),
    ];

    expect(
      groupByOrigin(library).map((g) => [g.title, g.skills.map((s) => s.name)]),
    ).toEqual([
      ["coreyhaines31/marketingskills", ["copywriting"]],
      ["JuliusBrussee/caveman", ["caveman"]],
      ["mattpocock/skills", ["diagnose", "tdd"]],
      ["Local skills", ["mine", "zzz-notes"]],
    ]);
  });

  it("has no local group when every skill has an Origin", () => {
    expect(groupByOrigin([skill("tdd", "mattpocock/skills")]).map((g) => g.title)).toEqual([
      "mattpocock/skills",
    ]);
  });
});
