<script>
  let { success } = $props();
</script>

{#if success > 0}
    <div class="toast success" role="status">
        Submit successful
    </div>
{/if}

<style>
    @keyframes disappear {
        0% {
            opacity: 1;
        }
        80% {
            opacity: 1;
        }
        100% {
            opacity: 0;
        }
    }

    .toast {
        position: absolute;
        left: 50%;
        bottom: 10px;
        padding: 0.5em;
        background-color: #dcfce7;
        border: 1px solid #86c99b;
        color: #166534;
        border-radius: 7px;
        transform: translateX(-50%);
        font-weight: bold;
        pointer-events: none;
        animation: disappear 2s linear forwards;
    }
</style>