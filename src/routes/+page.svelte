<script>
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import Overlay from "$lib/overlay.svelte";
  import ThumbnailGrid from "$lib/thumbnail-grid.svelte";
  import LoadingScreen from "$lib/loading-screen.svelte";
  import "./+layout.css"

  let selected = $state(null);
  let folderOpen = $state(false);

  async function openFolder() {
    const path = await open({
      directory: true,
      multiple: false,
      title: "Open folder"
    });

    if (!path) return;

    await invoke("set_root", { path });
    await loadImages();
    folderOpen = true;
  }

  let images = $state([]);

  async function loadImages() {
    console.log("Load images called")
    try {
      images = await invoke("get_images_as_strings");
    } catch (error) {
      console.error("Failed to load images:", error);
    }
  }

  let loading = $state(false);

  async function stamp() {
    if (images.length != 0) {
      loading = true;
      try {
        await invoke("stamp_root_images_noblock");
        await loadImages();
      } catch (error) {
        console.error("Failed to stamp images:", error);
      } finally {
        loading = false;
      }
    } else {
      console.error("Cannot stamp without images");
    }
  }

  async function dejunk() {
    if (images.length != 0) {
      loading = true;
      try {
        await invoke("dejunk_image_titles_noblock");
        await loadImages();
      } catch (error) {
        console.error("Failed to dejunk images:", error);
      } finally {
        loading = false;
      }
    } else {
      console.error("Cannot dejunk without images");
    }
  }

  let videoExtensions = $state([]);

  onMount(async () => {
      videoExtensions = await invoke("accepted_extensions_video");
  });

</script>

<main>
  <ThumbnailGrid selectElement={(element) => selected = element} images={images} videoExtensions={videoExtensions}/>
  {#key selected}  
  <Overlay selected={selected} onClose={() => selected = null} onNext={() => selected = selected?.nextElementSibling} onPrevious={() => selected = selected?.previousElementSibling} videoExtensions={videoExtensions}/>
  {/key}
  <LoadingScreen loading={loading}></LoadingScreen>
  <button onclick={openFolder} disabled={loading}>open folder</button>
  <button onclick={stamp} disabled={loading || !folderOpen}>stamp images</button>
  <button onclick={dejunk} disabled={loading || !folderOpen}>dejunk image titles</button>
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