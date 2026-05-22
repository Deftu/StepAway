<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import { listen } from '@tauri-apps/api/event';
    import { fade } from 'svelte/transition';
    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

    let remaining = $state(15);
    let active = $state(false);

    let listenTickCallback: () => void;
    let listenHideCallback: () => void;

    const window = getCurrentWebviewWindow();

    onMount(async () => {
        listenTickCallback = await listen<number>('warning-tick', (event) => {
            remaining = event.payload;
            active = true;
        });

        listenHideCallback = await listen('warning-hide', () => {
            active = false;
        });
    });

    onDestroy(() => {
        if (listenTickCallback) listenTickCallback();
        if (listenHideCallback) listenHideCallback();
    });

    async function onFadeComplete() {
        if (!active) {
            await window.hide();
        }
    }
</script>

{#if active}
    <main class="overlay" transition:fade={{ duration: 300 }} onoutroend={onFadeComplete}>
        <div class="toast">
            <span class="alert-text">Sleeping in {remaining}s</span>
            <span class="sub-text">Wiggle mouse to cancel</span>
        </div>
    </main>
{/if}

<style lang="scss">
    :global(html),
    :global(body),
    :global(.svelte-body) {
        background: transparent !important;
        overflow: hidden;
    }

    .overlay {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100vw;
        height: 100vh;
    }

    .toast {
        background: rgba(20, 20, 20, 0.85);
        backdrop-filter: blur(8px);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 12px;
        padding: 12px 24px;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4px;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    }

    .alert-text {
        font-family: "Inter", sans-serif;
        color: var(--dfg-danger, #ff4c4c);
        font-size: 16px;
        font-weight: 700;
        letter-spacing: 0.5px;
    }

    .sub-text {
        font-family: "Inter", sans-serif;
        color: #a0a0a0;
        font-size: 12px;
        font-weight: 500;
    }
</style>