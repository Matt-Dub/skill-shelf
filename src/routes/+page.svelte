<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { openPath } from "@tauri-apps/plugin-opener";
  import SkillCard from "$lib/SkillCard.svelte";
  import type { CardAction, Skill } from "$lib/library-view-model";

  function runAction(action: CardAction) {
    openPath(action.target).catch((error) => console.error(`${action.kind} failed`, error));
  }

  const library = invoke<Skill[]>("list_skills");
</script>

<main>
  <header>Skill Shelf</header>
  {#await library}
    <p class="status">Loading skills…</p>
  {:then skills}
    {#if skills.length === 0}
      <p class="status">No user skills found.</p>
    {:else}
      <ul>
        {#each skills as skill (skill.path)}
          <SkillCard {skill} onAction={runAction} />
        {/each}
      </ul>
    {/if}
  {:catch error}
    <p class="status">Could not read skills: {error}</p>
  {/await}
</main>

<style>
  :global(html, body) {
    margin: 0;
    font-family: system-ui, -apple-system, "Segoe UI", sans-serif;
    font-size: 13px;
    color: #1f1f1f;
    background: #fafafa;
  }
  @media (prefers-color-scheme: dark) {
    :global(html, body) {
      color: #ececec;
      background: #1e1e1e;
    }
  }
  main {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }
  header {
    padding: 10px 12px;
    font-weight: 600;
    border-bottom: 1px solid color-mix(in srgb, currentColor 12%, transparent);
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 4px 0;
    overflow-y: auto;
  }
  .status {
    padding: 12px;
    opacity: 0.7;
  }
</style>
