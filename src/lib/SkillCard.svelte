<script lang="ts">
  import { cardActions, type CardAction, type Skill } from "./library-view-model";

  let { skill, onAction }: { skill: Skill; onAction: (action: CardAction) => void } = $props();

  let expanded = $state(false);
</script>

<li class:expanded>
  <button class="toggle" aria-expanded={expanded} onclick={() => (expanded = !expanded)}>
    <span class="name">{skill.name}</span>
    <span class="description">{skill.description}</span>
  </button>
  {#if expanded}
    <div class="actions">
      {#each cardActions(skill) as action (action.kind)}
        <button onclick={() => onAction(action)}>{action.label}</button>
      {/each}
    </div>
  {/if}
</li>

<style>
  li {
    padding: 0;
  }
  li:hover,
  li.expanded {
    background: color-mix(in srgb, currentColor 6%, transparent);
  }
  .toggle {
    all: unset;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 100%;
    padding: 6px 12px;
    cursor: pointer;
  }
  .toggle:focus-visible {
    outline: 2px solid color-mix(in srgb, currentColor 40%, transparent);
    outline-offset: -2px;
  }
  .name {
    font-weight: 600;
  }
  .description {
    opacity: 0.7;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .expanded .description {
    display: block;
  }
  .actions {
    display: flex;
    gap: 6px;
    padding: 0 12px 8px;
  }
  .actions button {
    font: inherit;
    font-size: 12px;
    padding: 3px 8px;
    border-radius: 4px;
    border: 1px solid color-mix(in srgb, currentColor 20%, transparent);
    background: transparent;
    color: inherit;
    cursor: pointer;
  }
  .actions button:hover {
    background: color-mix(in srgb, currentColor 10%, transparent);
  }
</style>
