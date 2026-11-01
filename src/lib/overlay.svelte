<script>
    let { selectedImage, onClose } = $props();
</script>

{#if selectedImage}
<button class="overlay-blanket" onclick={onClose}>
    <div class="overlay-content">
    <div id="image-view">
        <div id="image-large">
        <!-- <img src="/pic.HEIC" alt="" /> -->
        </div>
        <div id="image-navigation"></div>
    </div>
    <div>
        <ul id="date-list">
        <li>2026-04-03-2345</li>
        <li>2026-04-03-2345</li>
        <li>2026-04-03-2345</li>
        </ul>
        <p>{selectedImage.src}</p>
        <form action="" id="decorate-form">
        <input type="text" name="Title" placeholder="Title" id="" />
        <input type="text" name="Title" placeholder="Tags" id="" />
        </form>
    </div>
    </div>
</button>
{/if}

<style>
  .overlay-blanket {
    position: fixed;
    top: 10vh;
    left: 10vw;
    height: 80vh;
    width: 80vw;
    border: 1px solid #eeeeee;
    background-color: white;
    border-radius: 16px;
    box-shadow: 0 3px 4px rgba(0, 0, 0, 0.5);
  }
</style>