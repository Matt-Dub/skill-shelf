import { describe, expect, it } from "vitest";
import { cardActions, type Skill } from "./library-view-model";

const tdd: Skill = {
  name: "tdd",
  description: "Test-driven development.",
  path: "/home/ada/.claude/skills/tdd",
};

describe("cardActions", () => {
  it("offers both local actions for a skill on disk", () => {
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
