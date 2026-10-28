<script>
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container">
  <div class="grid">
    {#each { length: 40 }, rank}
      <button aria-label="Open image">
        <img src="/pic.HEIC" class="thumbnail" alt="something"/>
      </button>
    {/each}
  </div>

  <!-- <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p> -->
</main>

<style>

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.thumbnail {
  display: block;
  width: 100%;
  /* height: 6em; */
  padding: 0em;
  will-change: filter;
  transition: 0.75s;
}

.row {
  display: flex;
  justify-content: center;
}

button {
  /* border-radius: 8px; */
  border: 1px solid transparent;
  padding: 0px;
  margin: 1em;
  width: 20%;
  /* font-size: 1em; */
  /* font-weight: 500; */
  /* font-family: inherit; */
  /* color: #0f0f0f; */
  background-color: #ffffff;
  /* transition: border-color 0.25s; */
  /* box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2); */
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

</style>
