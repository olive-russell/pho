<script>
  let { src, alt, path, videoExtensions, autoplayLoop, muted } = $props();
</script>

{#if !videoExtensions.includes(path.split(".").pop().toLowerCase())}
<img class="image" src="{src}" alt="{alt}" />
{:else}
<!-- svelte-ignore a11y_media_has_caption -->
<video class="image" src="{src}" alt="{alt}" autoplay={autoplayLoop} loop={autoplayLoop} muted={muted}></video>
{/if}

<style>
  .image {
    object-fit: contain;
    max-width: 100%;
    max-height: 100%;
    width: auto;
    height: auto;
  }
</style>