<script>
    import { convertFileSrc } from "@tauri-apps/api/core";
    import Media from "./media.svelte";
    let { selectElement, images, videoExtensions } = $props();
</script>

<section id="image-list" class="grid">
    {#each images as image}
    <button class="image-tile" aria-label="Open image" onclick={(event) => selectElement(event.currentTarget)}>
      <figure>
        <Media src={convertFileSrc(image)} alt={image.split("/").pop()} path={image} videoExtensions={videoExtensions} clickable=true></Media>
        <figcaption>{image.split("/").pop()}</figcaption>
      </figure>
    </button>
    {/each}
</section>

<style>
  #image-list {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: center;
  }

  #image-list>button {
    border: none;
    padding: 0px;
    margin: 1em;
    width: 20%;
    background-color: transparent;
    cursor: pointer;
    height: 100%;
  }

  #image-list>button:focus {
    outline: none;
  }
  
  #image-list>button>figure {
    margin: auto;
  }
  
  #image-list>button>figure>figcaption {
    margin-top: 0.5em;
    line-break: anywhere;
  }
</style>