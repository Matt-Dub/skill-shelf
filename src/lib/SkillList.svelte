<script lang="ts">
  import SkillCard from "./SkillCard.svelte";
  import { groupByOrigin, type CardAction, type Skill } from "./library-view-model";

  let { skills, onAction }: { skills: Skill[]; onAction: (action: CardAction) => void } = $props();

  const groups = $derived(groupByOrigin(skills));
</script>

{#each groups as group, index (group.title)}
  <section aria-labelledby="group-{index}">
    <h2 id="group-{index}">{group.title}</h2>
    <ul>
      {#each group.skills as skill (skill.path)}
        <SkillCard {skill} {onAction} />
      {/each}
    </ul>
  </section>
{/each}

<style>
  h2 {
    margin: 0;
    padding: 10px 12px 4px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    opacity: 0.55;
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 0 0 4px;
  }
</style>
