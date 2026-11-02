<script>
    import { convertFileSrc } from "@tauri-apps/api/core";
    let { selectElement, images } = $props();
</script>

<section id="image-list" class="grid">
    {#each images as image}
        <button class="image-tile" aria-label="Open image" onclick={(event) => selectElement(event.currentTarget)}>
            <figure>
                <img src="{convertFileSrc(image)}" alt="{image}" />
                <figcaption>{image}</figcaption>
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

  #image-list:focus-within img:hover {
    border-color: white;
  }
  
  #image-list>button>figure {
    margin: auto;
  }
  
  #image-list>button>figure>figcaption {
    margin-top: 0.5em;
  }
</style>