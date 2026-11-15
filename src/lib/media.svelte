<script>
  let { src, alt, path, videoExtensions, autoplayLoop, muted, clickable = false } = $props();
</script>
{console.log({ path, clickable })}
{#if !videoExtensions.includes(path.split(".").pop().toLowerCase())}
<img class="image" src="{src}" alt="{alt}" path={path} class:clickable />
{:else}
<!-- svelte-ignore a11y_media_has_caption -->
<video class="image" src="{src}" alt="{alt}" path={path} autoplay={autoplayLoop} loop={autoplayLoop} muted={muted} class:clickable></video>
{/if}

<style>
  .image {
    display: block;
    object-fit: contain;
    max-width: 100%;
    max-height: 100%;
    width: auto;
    height: auto;
    border: 2px solid transparent;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    border-radius: 8px;
    /* width: 100%; */
    box-sizing: border-box;
    padding: 0em;
    transition: border 0.3s;
  }

  .image.clickable:hover{
    border-color: #396cd8;
    transition: 0.1s;
  }
</style>