<script>
    let { selectImage } = $props();
</script>

<section id="image-list" class="grid">
    {#each { length: 40 }, rank}
        <button class="image-tile" aria-label="Open image" onclick={(event) => selectImage(event.target)}>
            <figure>
                <img src="demo-pics/pic{rank.toString(10).padStart(2, '0')}.HEIC" alt="something" />
                <figcaption>pic{rank.toString(10).padStart(2, '0')}.HEIC</figcaption>
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
  
  #image-list>button>figure {
    margin: auto;
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
</style>