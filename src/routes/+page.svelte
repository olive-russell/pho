<script>
  import { invoke } from "@tauri-apps/api/core";
  import Overlay from "$lib/overlay.svelte";
  import ThumbnailGrid from "$lib/thumbnail-grid.svelte";

  let selectedImage = $state(null);
</script>

<main>
  <ThumbnailGrid selectImage={(element) => selectedImage = element}/>
  <Overlay selectedImage={selectedImage} onClose={() => selectedImage = null}/>
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
</style>