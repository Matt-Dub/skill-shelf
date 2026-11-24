<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { openPath, openUrl } from "@tauri-apps/plugin-opener";
  import SkillList from "$lib/SkillList.svelte";
  import type { CardAction, Skill } from "$lib/library-view-model";

  function runAction(action: CardAction) {
    const open = action.kind === "open-origin" ? openUrl : openPath;
    open(action.target).catch((error) => console.error(`${action.kind} failed`, error));
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
      <div class="library">
        <SkillList {skills} onAction={runAction} />
      </div>
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
  .library {
    overflow-y: auto;
  }
  .status {
    padding: 12px;
    opacity: 0.7;
  }
</style>
