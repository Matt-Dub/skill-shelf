import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import SkillCard from "./SkillCard.svelte";

const skill = {
  name: "tdd",
  description: "Test-driven development. Use when the user wants to build features test-first.",
  path: "/home/ada/.claude/skills/tdd",
  origin: null,
};

describe("SkillCard", () => {
  it("expands on click to show its actions, and collapses on a second click", async () => {
    const user = userEvent.setup();
    render(SkillCard, { skill, onAction: () => {} });
    const toggle = screen.getByRole("button", { name: /tdd/ });

    expect(toggle).toHaveAttribute("aria-expanded", "false");
    expect(screen.queryByRole("button", { name: "Open folder" })).not.toBeInTheDocument();

    await user.click(toggle);
    expect(toggle).toHaveAttribute("aria-expanded", "true");
    expect(screen.getByRole("button", { name: "Open folder" })).toBeVisible();
    expect(screen.getByRole("button", { name: "Open SKILL.md" })).toBeVisible();

    await user.click(toggle);
    expect(toggle).toHaveAttribute("aria-expanded", "false");
    expect(screen.queryByRole("button", { name: "Open folder" })).not.toBeInTheDocument();
  });

  it("hands the chosen action to onAction", async () => {
    const user = userEvent.setup();
    const onAction = vi.fn();
    render(SkillCard, { skill, onAction });

    await user.click(screen.getByRole("button", { name: /tdd/ }));
    await user.click(screen.getByRole("button", { name: "Open SKILL.md" }));

    expect(onAction).toHaveBeenCalledExactlyOnceWith({
      kind: "open-skill-md",
      label: "Open SKILL.md",
      target: "/home/ada/.claude/skills/tdd/SKILL.md",
    });
  });
});
