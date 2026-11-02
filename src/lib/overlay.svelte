<script>
  import { event } from "@tauri-apps/api";
  import Tags from 'svelte-tags-input';
  
  let { selected, onClose, onNext, onPrevious } = $props();
  
  // Handle dialog state and actions
  let dialog = $state(null);
  $effect(() => {
    if (selected && !dialog.open) {
      dialog.showModal();
    }
  });
  
  function handleDialogKeydown(event) {
    if (event.key === "Escape") {
      event.preventDefault();
      dialog.close();
    }
  }
  
  // Derive selected image and siblings
  let selectedImage = $derived(selected?.querySelector("img") ?? null);
  let previousImage = $derived(selected?.previousElementSibling?.querySelector("img") ?? null);  
  let nextImage = $derived(selected?.nextElementSibling?.querySelector("img") ?? null);
  
  // Configure title input
  // svelte-ignore state_referenced_locally
  let title = $state(selectedImage.src)
  
  // Configure tag input
  let availableTags = $state(['one', 'two']);
  let tags = $state([]);
</script>

{#if selected}
<dialog bind:this={dialog} onclose={onClose} onkeydown={handleDialogKeydown} onclick={(event) => {if (event.target === dialog) dialog.close()}}>
  <div id="left">
    <figure>
      <div>
        <img src="{selectedImage.src}" alt="" />
      </div>
      <figcaption>{selectedImage.src.split("/").pop()}</figcaption>
    </figure>
    <nav>
      {#if previousImage}
      <button onclick={onPrevious}>
        <img src="{previousImage.src}" alt="">
        <span>Previous</span>
      </button>
      {:else}
      <button disabled>Previous</button>
      {/if}
      {#if nextImage}
      <button onclick={onNext}>
        <span>Next</span>
        <img src="{nextImage.src}" alt="">
      </button>
      {:else}
      <button disabled>Next</button>
      {/if}
    </nav>
  </div>
  <div id="right">
    <ul id="details">
      <li>2026-04-03-2345</li>
      <li>2026-04-03-2345</li>
      <li>2026-04-03-2345</li>
    </ul>
    <form id="form">
      <input type="text" name="Title" placeholder="{title}" id="" />
      <Tags bind:tags autoComplete={availableTags} onlyUnique placeholder="Tags" />
      <button type="submit">Save</button>
    </form>
  </div>
</dialog>
{/if}

<style>
  dialog {
    height: 80vh;
    width: 75vw;
    max-height: 80vh;
    border: 2px solid #eeeeee;
    background-color: white;
    border-radius: 16px;
    box-shadow: 0 3px 4px rgba(0, 0, 0, 0.5);
  }
  
  dialog::backdrop {
    background: rgba(0, 0, 0, 0.2);
  }
  
  dialog {
    display: flex;
  }

  #left,
  #right {
    display: flex;
    flex: 1;
    flex-direction: column;
    
    min-width: 0;
    min-height: 0;
  }
  
  figure {
    flex: 0 0 80%;
    min-height: 0;
    margin: 0em;
    display: flex;
    flex-direction: column;
  }
  
  figure>div {
    flex: 1;
    min-height: 0;
    min-width: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  figure>div>img {
    display: block;
    max-width: 100%;
    max-height: 100%;
    width: auto;
    height: auto;
    border-radius: 8px;
  }
  
  figure>div>img:hover {
    border-color: white;
  }
  
  figure>figcaption {
    margin-top: 1em;
  }
  
  nav {
    display: flex;
    flex: 0 0 20%;
    flex-direction: row;
    justify-content: center;
    gap: 1em;
    min-height: 0;
    /* align-items: stretch */
  }
  
  nav>button {
    display: flex;
    align-items: center;
    margin: 1em;
    width: 50%;
    background: none;
    border: none;
  }
  
  nav>button:nth-child(2) {
    justify-content: right;
  }
  
  nav>button>img {
    object-fit: contain;
    max-width: 100%;
    max-height: 100%;
    width: auto;
    height: auto;
  }
  
  nav>button>span {
    margin: 1em;
    font-size: large;
  }
  
  ul {
    padding-left: 0px;
  }
  
  li {
    display: flex;
    justify-content: center;
  }
</style>