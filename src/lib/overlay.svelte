<script>
  import { invoke } from "@tauri-apps/api/core";
  import { event } from "@tauri-apps/api";
  import Tags from 'svelte-tags-input';
  import { onMount } from "svelte";
  import Media from "./media.svelte";
  
  let { selected, onClose, onNext, onPrevious, onSuccess, showSuccess, videoExtensions } = $props();
  
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
  let selectedImage = $derived(selected?.querySelector(".image") ?? null);
  let previousImage = $derived(selected?.previousElementSibling?.querySelector(".image") ?? null);  
  let nextImage = $derived(selected?.nextElementSibling?.querySelector(".image") ?? null);
  
  // Get list of dates
  let dateTaken = $derived(metadata ? metadata.date_taken : "");
  let dateCreated = $derived(metadata ? metadata.date_created : "");
  let dateMediaCreated = $derived(metadata ? metadata.date_media_created : "");

  let metadata = $state(null);
  $effect(() => {
    if (selectedImage) extractMetadata(selectedImage.getAttribute("path"));
  });
    
  async function extractMetadata(pathString) {
    try {
      metadata = await invoke("extract_metadata_from_string", {pathString});
    } catch (error) {
      console.error("Failed to extract metadata:", error);
    }
  }

  // Configure stamp input
  let stamp = $state(null);
  $effect(() => {
    if (selected) parseStamp(selectedImage.getAttribute("name"));
  });
    
  async function parseStamp(pathString) {
    try {
      stamp = await invoke("parse_stamp_from_string", {pathString});
    } catch (error) {
      console.error("Failed to parse stamp:", error);
    }
  }

  // Configure title input
  // svelte-ignore state_referenced_locally
  let title = $state(null);
  $effect(() => {
    if (selected) parseTitle(selectedImage.getAttribute("name"));
  });
  
  async function parseTitle(pathString) {
    try {
      title = await invoke("parse_title_from_string", {pathString}) ?? "";
    } catch (error) {
      console.error("Failed to parse title:", error);
    }
  }

  // Configure tag input
  let availableTags = $state(['Olive','F']);
  let tags = $state([]);
  $effect(() => {
    if (selected) parseTags(selectedImage.getAttribute("name"));
  });
  
  async function parseTags(pathString) {
    tags = []; //TODO can i turn this line off?
    try {
        const result = await invoke("parse_tags_from_string", {pathString});
        tags = Array.isArray(result) ? result : [];
    } catch (error) {
        console.error("Failed to parse tags:", error);
        tags = [];
    }
  }


  // Form submit handler
  async function handleSubmit(event) {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);

    const path = selectedImage.getAttribute("path");

    const action = event.submitter?.value ?? "save";

    const stampPattern = /^\d{4}-\d{2}-\d{2}-\d{4}-\d{4}$/;

    if (!stampPattern.test(stamp)) {
        console.error("Invalid stamp format");
        const input = form.elements.namedItem("stamp");
        input.setCustomValidity("Stamp does not meet format requirements 0000-00-00-0000-0000");
        input.reportValidity();
        input.addEventListener("input", () => {
          input.setCustomValidity("");
        });
        return;
    }

    const formAlert = document.querySelector("#form-alert");
    if (action == "save" || action == "save-next") {
      try {
        await invoke("save_annotation", {
          path,
          stamp,
          title,
          tags
        });

        if (action == "save-next") {
          onNext();
        }
        onSuccess();
        formAlert.textContent = "";
        formAlert.classList.add("hidden");
      } catch (error) {
        console.error("Failed to save annotation:", error);
        formAlert.textContent = error;
        formAlert.classList.remove("hidden");
      }
    }
  }

  async function loadAvailableTags() {
    try {
        const result = await invoke("load_photo_album_tags");
        availableTags = Array.isArray(result) ? result : [];
    } catch (error) {
        console.error("Failed to load existing tag list:", error);
        availableTags = [];
    }
  }

  onMount(() => {
    loadAvailableTags();
  });
</script>

{#if selected}
<dialog bind:this={dialog} onclose={onClose} onkeydown={handleDialogKeydown} onclick={(event) => {if (event.target === dialog) dialog.close()}}>
  <div id="left">
    <figure>
      <div>
        <Media src={selectedImage.src} alt={selectedImage.getAttribute("name")} name={selectedImage.getAttribute("name")} path={selectedImage.getAttribute("path")} videoExtensions={videoExtensions} autoplayLoop=true></Media>
      </div>
      <figcaption>{selectedImage.getAttribute("name")}</figcaption>
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
      <p id="form-alert" class="hidden" role="alert"></p>
      <Tags bind:tags autoComplete={availableTags} onlyUnique onlyAutocomplete placeholder="Tags" minChars=0 />
      <input type="text" name="Title" bind:value={title} placeholder="Title" id="title" />
      <input type="text" name="Stamp" bind:value={stamp} id="stamp" placeholder="{stamp}">
      <div id="actions">
        <button type="reset">Reset</button>
        <button type="submit" name="action" value="save-next">Save and next</button>
        <button type="submit" name="action" value="save">Save</button>
      </div>
    </form>
    <nav>
      {#if previousImage}
      <button onclick={onPrevious}>
        <Media src={previousImage.src} alt={previousImage.getAttribute("name")} name={previousImage.getAttribute("name")} path={previousImage.getAttribute("path")} videoExtensions={videoExtensions} autoplayLoop=true muted=true clickable=true></Media>
      </button>
      {:else}
      <button disabled>Previous</button>
      {/if}
      {#if nextImage}
      <button onclick={onNext}>
        <Media src={nextImage.src} alt={nextImage.getAttribute("name")} name={nextImage.getAttribute("name")} path={nextImage.getAttribute("path")} videoExtensions={videoExtensions} autoplayLoop=true muted=true clickable=true></Media>
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
    overflow: hidden;
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
  
  figure>figcaption {
    margin-top: 1em;
  }
  
  nav {
    display: flex;
    flex: 0 0 20%;
    flex-direction: row;
    justify-content: center;
    min-height: 0;
  }
  
  nav>button {
    position: relative;
    display: flex;
    align-items: center;
    width: 50%;
    background: none;
    border: none;
  }
  
  nav>button:nth-child(2) {
    justify-content: right;
  }

  nav>button::after {
    position:absolute;
    left: 15%;
    top: 50%;
    height: 100%;
    content: "←";
    font-size: 10em;
    line-height: 0px;
    font-weight: bold;
    font: arial;
    color: #ffffff55;
    pointer-events: none;
  }

  nav>button:nth-child(2)::after {
    content: "→"
  }

  #right {
    justify-content: space-between;
    font-size: large;
    padding: 1em;
    overflow: scroll;
  }
  
  dl {
    text-align: left;
    padding: 0em;
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
  }

  form * {
    width: 100%;
    padding: 0px;
    box-sizing: border-box;
    margin: 5px 0px;
    font-size: medium;
  }

  form input,
  form #form-alert {
    border: 1px solid #ccc;
    border-radius: 2px;
    padding: 0.6em;
  }

  form #form-alert {
    border: none;
    margin-bottom: 0.5em;
  }

  form #actions {
    display: flex;
    flex-direction: row;
  }

  form #actions button {
    flex: 1 1 100%;
    padding: 0.6em 0px;
  }

  #form-alert {
    background-color: #ff000033;
  }

  #form-alert.hidden {
    visibility: hidden;
  }

</style>