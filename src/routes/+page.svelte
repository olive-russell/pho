<script>
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import Overlay from "$lib/overlay.svelte";
  import ThumbnailGrid from "$lib/thumbnail-grid.svelte";
  import "./+layout.css"

  let selected = $state(null);

  async function openFolder() {
    const path = await open({
      directory: true,
      multiple: false,
      title: "Open folder"
    });

    if (!path) return;

    await invoke("set_root", { path });
    await loadImages();
  }

  let images = $state([]);

  async function loadImages() {
    try {
      images = await invoke("get_images_as_strings");
    } catch (error) {
      console.error("Failed to load images:", error);
    }
  }

  async function stamp() {
    if (images.length != 0) {
      await invoke("stamp_root_images");
    } else {
      console.error("Cannot stamp without images");
    }
  }

  async function dejunk() {
    if (images.length != 0) {
      await invoke("dejunk_image_titles");
    } else {
      console.error("Cannot dejunk without images");
    }
  }

</script>

<main>
  <ThumbnailGrid selectElement={(element) => selected = element} images={images}/>
  <Overlay selected={selected} onClose={() => selected = null} onNext={() => selected = selected?.nextElementSibling} onPrevious={() => selected = selected?.previousElementSibling}/>
  <button onclick={openFolder}>open folder</button>
  <button onclick={stamp}>stamp images</button>
  <button onclick={dejunk}>dejunk image titles</button>
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