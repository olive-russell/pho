<script>
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  let overlayOpen = $state(false);
  function toggleOverlay() {
    overlayOpen = !overlayOpen;
  }
</script>

<main>
  <div id="image-list" class="grid">
    {#each { length: 40 }, rank}
      <button class="image-tile" aria-label="Open image" onclick={toggleOverlay}>
        <figure>
          <img src="/pic.HEIC" alt="something" />
          <figcaption>pic.HEIC</figcaption>
        </figure>
      </button>
    {/each}
  </div>
  {#if overlayOpen}
    <button class="overlay-blanket" onclick={toggleOverlay}>
      <div class="overlay-content">
        <div id="image-view">
          <div id="image-large">
            <img src="/pic.HEIC" alt="" />
          </div>
          <div id="image-navigation"></div>
        </div>
        <div>
          <ul id="date-list">
            <li>2026-04-03-2345</li>
            <li>2026-04-03-2345</li>
            <li>2026-04-03-2345</li>
          </ul>
          <form action="" id="decorate-form">
            <input type="text" name="Title" placeholder="Title" id="" />
            <input type="text" name="Title" placeholder="Tags" id="" />
          </form>
        </div>
      </div>
    </button>
  {/if}

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

  main {
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  #image-list>button {
    border: none;
    padding: 0px;
    margin: 1em;
    width: 20%;
    background-color: transparent;
    cursor: pointer;
  }
  
  #image-list>button>figure {
    margin: 0px;
  }
  
  #image-list>button>figure>img {
    display: block;
    border: 2px solid transparent;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    border-radius: 8px;
    width: 100%;
    box-sizing: border-box;

    padding: 0em;
    will-change: filter;
    transition: border 0.3s;
  }

  #image-list>button>figure>img:hover {
    border-color: #396cd8;
    transition: 0.1s;
  }
  
  #image-list>button>figure>img:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  #image-list>button>figure>figcaption {
    margin-top: 0.5em;
  }

  .overlay-blanket {
    position: fixed;
    top: 10vh;
    left: 10vw;
    height: 80vh;
    width: 80vw;
    border: 1px solid #eeeeee;
    background-color: white;
    border-radius: 16px;
    box-shadow: 0 3px 4px rgba(0, 0, 0, 0.5);
  }
</style>
