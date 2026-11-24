import { describe, expect, it } from "vitest";
import { render, screen, within } from "@testing-library/svelte";
import SkillList from "./SkillList.svelte";
import type { Skill } from "./library-view-model";

const skill = (name: string, repo: string | null): Skill => ({
  name,
  description: `${name} description`,
  path: `/home/ada/.claude/skills/${name}`,
  origin: repo ? { url: `https://github.com/${repo}`, repo } : null,
});

describe("SkillList", () => {
  it("shows one titled section per Origin, with local skills last", () => {
    render(SkillList, {
      skills: [skill("diagnose", "mattpocock/skills"), skill("mine", null), skill("tdd", "mattpocock/skills")],
      onAction: () => {},
    });

    const headings = screen.getAllByRole("heading").map((h) => h.textContent);
    expect(headings).toEqual(["mattpocock/skills", "Local skills"]);

    const repo = screen.getByRole("region", { name: "mattpocock/skills" });
    expect(within(repo).getAllByRole("button").map((b) => b.textContent)).toEqual([
      expect.stringContaining("diagnose"),
      expect.stringContaining("tdd"),
    ]);
    const local = screen.getByRole("region", { name: "Local skills" });
    expect(within(local).getByRole("button", { name: /mine/ })).toBeInTheDocument();
  });
});
