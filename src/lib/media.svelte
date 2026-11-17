<script>
  let { src, alt, path, name, videoExtensions, autoplayLoop, muted, clickable = false } = $props();

  let loaded = $state(false);
</script>
{#if !videoExtensions.includes(path.split(".").pop().toLowerCase())}
<img class="image" src="{src}" alt="{alt}" name={name} path={path} class:clickable loading="lazy" />
{:else}
<!-- svelte-ignore a11y_media_has_caption -->
<video class="image" src="{src}" alt="{alt}" name={name} path={path} autoplay={autoplayLoop} loop={autoplayLoop} muted={muted} class:clickable onloadeddata={() => loaded = true} class:loaded></video>
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

  video.image:not(.loaded) {
    visibility: hidden;
  }
</style>