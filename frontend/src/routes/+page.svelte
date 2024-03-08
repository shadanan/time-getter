<script lang="ts">
  let time: Promise<Time> | null = null;

  type Time = {
    time: string;
  };

  async function fetchTime(): Promise<Time> {
    const response = await fetch("/api/time");
    const result = await response.json();
    return result;
  }
</script>

<h1>Time Getter</h1>

<button on:click={() => (time = fetchTime())}>Get Time</button>

{#if time}
  {#await time}
    <div>Fetching time...</div>
  {:then time}
    <div>Current time: {time.time}</div>
  {:catch err}
    <div>Failed to fetch time: {err}</div>
  {/await}
{/if}
