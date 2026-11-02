<script>
  import { invoke } from "@tauri-apps/api/core";
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
  
  // Get list of dates
  let dateTaken = $derived(selectedImage ? "2024-12-04-1906" : "");
  let dateCreated = $derived(selectedImage ? "2025-01-11-2050" : "");
  let dateMediaCreated = $derived(selectedImage ? "2024-12-04-1906" : "");

  // Configure date input
  let date = $derived(selectedImage ? dateTaken : "")

  // Configure title input
  // svelte-ignore state_referenced_locally
  let title = $derived(selectedImage?.src.split("/").pop() ?? "");
  
  // Configure tag input
  let availableTags = $state(['one', 'two']);
  let tags = $state(['one']);

  // Form submit handler
  async function handleSubmit(event) {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);

    const action = event.submitter?.value ?? "save";

    try {
      await invoke("save_annotation", {
        date,
        title,
        tags,
        action
      });
    } catch (error) {
      console.error("Failed to save annotation:", error);
    }
  }
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
  </div>
  <div id="right">
    <dl id="details">
      <dt>Date taken:</dt>
      <dd>{dateTaken}</dd>
      <dt>Date created:</dt>
      <dd>{dateCreated}</dd>
      <dt>Date media created:</dt>
      <dd>{dateMediaCreated}</dd>
    </dl>
    <form id="form" onsubmit={handleSubmit}>
    <input type="datetime" name="Date" bind:value={date} id="date" placeholder="{date}">
      <input type="text" name="Title" bind:value={title} placeholder="{title}" id="" />
      <Tags bind:tags autoComplete={availableTags} onlyUnique onlyAutocomplete placeholder="Tags" minChars=0 />
      <div id="actions">
        <button type="reset">Reset</button>
        <button type="submit" name="action" value="save-next">Save and next</button>
        <button type="submit" name="action" value="save">Save</button>
      </div>
    </form>
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

  #left {
    flex-basis: 60%;
  }

  #right {
    flex-basis: 40%;
  }
  
  figure {
    flex: 0 0 100%;
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

  #right *,
  #right {
    /* outline: 1px solid red; */
  }

  #right {
    justify-content: space-around;
    font-size: large;
  }

  dl {
    text-align: left;
    padding: 1em;
  }

  dt, dd {
    display: inline;
    margin: 0;
  }

  dt {
    font-weight: bold;
  }

  dd::after {
    content: "";
    display: block;
    margin-bottom: 0.5em;
  }

  form {
    display: flex;
    flex-direction: column;
    /* align-items: center; */
  }

  form * {
    width: 100%;
    padding: 0px;
    box-sizing: border-box;
    margin: 5px 0px;
    font-size: medium;
    /* border-radius: 5px; */
  }

  form input{
    /* border-radius: 1px; */
    border: 1px solid #ccc;
    border-radius: 2px;
  }

  form input {
    padding: 0.6em;
  }

  form #actions {
    display: flex;
    flex-direction: row;
  }

  form #actions button {
    flex: 1 1 100%;
    padding: 0.6em 0px;
    /* border-radius: 2px; */
  }

</style>