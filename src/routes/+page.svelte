<script>
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import Overlay from "$lib/overlay.svelte";
  import ThumbnailGrid from "$lib/thumbnail-grid.svelte";
  import LoadingScreen from "$lib/loading-screen.svelte";
  import SuccessToast from "$lib/success-toast.svelte";
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
  
  let success = $state(0);
  async function onSuccess(renamed, renamedImage, newPath) {
    // Update changed image
    onRename(renamedImage.getAttribute("path"), newPath);

    // Display success spop-up
    success += 1;
    setTimeout(() => {
      success -= 1;
    }, 2000);
  }

  let videoExtensions = $state([]);

  onMount(async () => {
      videoExtensions = await invoke("accepted_extensions_video");
  });


  onMount(() => {
    let unlisten;
    let cancelled = false;

    listen("image-renamed", (event) => {
        const { oldPath, newPath } = event.payload;

        // Update your image state here
        onRename(oldPath, newPath);
    }).then(fn => {
        if (cancelled) fn();
        else unlisten = fn;
    });

    return () => {
        cancelled = true;
        unlisten?.();
    };
  });

  function onRename(oldPath, newPath) {
    images = images.map(path =>
        path === oldPath ? newPath : path
    );
  }

</script>

<main>
  <ThumbnailGrid selectElement={(element) => selected = element} images={images} videoExtensions={videoExtensions}/>
  {#key selected}  
  <Overlay selected={selected} onClose={() => selected = null} onNext={() => selected = selected?.nextElementSibling} onPrevious={() => selected = selected?.previousElementSibling} onSuccess={onSuccess} videoExtensions={videoExtensions}/>
  {/key}
  <LoadingScreen loading={loading}></LoadingScreen>
  {#key success}
  <SuccessToast success={success}></SuccessToast>
  {/key}
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