<script>
  let { loading } = $props();
</script>

{#if loading}
<div class="loading-overlay">
    <div class="loading-box">
        Processing images...
    </div>
</div>
{/if}

<style>
    .loading-overlay {
        position: fixed;
        inset: 0;
        z-index: 1000;
        display: grid;
        place-items: center;
        background: rgb(0 0 0 / 40%);
    }
    
    .loading-box {
        padding: 2rem 3rem;
        background: white;
        border-radius: 8px;
    }
</style>