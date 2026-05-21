<script lang="ts">
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { writable } from "svelte/store";
    import { checkForUpdates, performUpdate } from "$lib/updater";
    import type { Update } from "@tauri-apps/plugin-updater";

    const window = getCurrentWebviewWindow();

    let isChecking = false;
    let isUpdating = false;
    let availableUpdate: Update | null = null;
    let downloadProgress = writable(0);

    $: tooltipText = isUpdating
        ? `Downloading... ${$downloadProgress}%`
        : availableUpdate
        ? `Install Update v${availableUpdate.version}`
        : isChecking
        ? "Checking for updates..."
        : "Check for Updates";

    async function handleUpdateClick() {
        if (isUpdating) return;

        if (availableUpdate) {
            isUpdating = true;
            try {
                await performUpdate(availableUpdate, downloadProgress);
            } catch (error) {
                console.error("Update failed:", error);
                isUpdating = false;
            }
        } else {
            isChecking = true;
            try {
                availableUpdate = await checkForUpdates();
            } finally {
                isChecking = false;
            }
        }
    }

    async function minimize() {
        await window.minimize();
    }

    async function close() {
        await window.hide();
    }
</script>

<nav class="title-bar" data-tauri-drag-region>
    <h2 class="dfg-header-2">Step Away</h2>

    <div class="title-bar-controls">
        <!-- Updater Button -->
        <button
            class="dfg-button update-btn {isChecking || isUpdating ? 'spinning' : ''} {availableUpdate && !isUpdating ? 'has-update' : ''}" 
            title={tooltipText}
            onclick={handleUpdateClick}
        >
            {#if availableUpdate && !isUpdating}
                <!-- Download Icon (Shows when update is ready) -->
                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                    <polyline points="7 10 12 15 17 10"></polyline>
                    <line x1="12" y1="15" x2="12" y2="3"></line>
                </svg>
            {:else}
                <!-- Sync Icon (Shows for default and checking states) -->
                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M21 2v6h-6"></path>
                    <path d="M3 12a9 9 0 0 1 15-6.7L21 8"></path>
                    <path d="M3 22v-6h6"></path>
                    <path d="M21 12a9 9 0 0 1-15 6.7L3 16"></path>
                </svg>
            {/if}
        </button>

        <!-- Minimize -->
        <button class="dfg-button" title="Minimize" onclick={minimize}>
            <svg xmlns="http://www.w3.org/2000/svg" width="10" height="2" viewBox="0 0 10 2" fill="none">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M0 0.375H10V1.625H0V0.375Z" fill="currentColor" />
            </svg>
        </button>

        <!-- Close -->
        <button class="dfg-button" title="Close" onclick={close}>
            <svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 10 10" fill="none">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M9.18798 10L8.76226e-05 0.812103L0.812191 0L10.0001 9.1879L9.18798 10Z" fill="currentColor" />
                <path fill-rule="evenodd" clip-rule="evenodd" d="M0.812104 10L10 0.812103L9.1879 0L0 9.1879L0.812104 10Z" fill="currentColor" />
            </svg>
        </button>
    </div>
</nav>

<style lang="scss">
    .title-bar {
        background: var(--dfg-background-2);
        position: sticky;
        top: 0;
        display: flex;
        width: 100%;
        padding: 10px 25px;
        justify-content: space-between;
        align-items: center;

        .title-bar-controls {
            display: flex;
            padding: 10px;
            justify-content: center;
            align-items: center;
            gap: 20px;

            .dfg-button {
                background: var(--dfg-background-1);
                transition: all 0.2s ease;

                &:hover {
                    background: var(--dfg-background-2);
                    filter: brightness(1.1);
                }
            }

            svg {
                color: var(--dfg-text);
            }

            .update-btn {
                &.has-update {
                    svg {
                        color: var(--dfg-primary);
                    }
                }

                &.spinning {
                    svg {
                        animation: spin 1.2s cubic-bezier(0.4, 0, 0.2, 1) infinite;
                        color: var(--dfg-primary-variant);
                    }
                }
            }
        }
    }

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }
</style>