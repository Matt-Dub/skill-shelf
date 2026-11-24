<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type Skill = { name: string; description: string; path: string };

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
          <li title={skill.description}>
            <span class="name">{skill.name}</span>
            <span class="description">{skill.description}</span>
          </li>
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
  li {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 6px 12px;
  }
  li:hover {
    background: color-mix(in srgb, currentColor 6%, transparent);
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
  .status {
    padding: 12px;
    opacity: 0.7;
  }
</style>
