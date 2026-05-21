<script lang="ts">
    interface Option {
        value: string;
        label: string;
    }

    let {
        options,
        value = $bindable(),
        ariaLabelledby = ""
    }: {
        options: Option[];
        value: string;
        ariaLabelledby?: string;
    } = $props();

    let isOpen = $state(false);

    let currentLabel = $derived(options.find(opt => opt.value === value)?.label || 'Select...');

    function toggle(e: Event) {
        e.stopPropagation();
        isOpen = !isOpen;
    }

    function select(val: string) {
        value = val;
        isOpen = false;
    }

    function close() {
        isOpen = false;
    }
</script>

<svelte:window onclick={close} />

<div class="custom-select-wrapper">
    <button
        class="custom-select-trigger {isOpen ? 'active' : ''}"
        aria-labelledby={ariaLabelledby}
        onclick={toggle}
    >
        {currentLabel}
        <svg
            class="chevron {isOpen ? 'open' : ''}"
            xmlns="http://www.w3.org/2000/svg"
            width="16" height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <polyline points="6 9 12 15 18 9"></polyline>
        </svg>
    </button>

    {#if isOpen}
        <div class="custom-select-menu">
            {#each options as option}
                <button
                    class="custom-select-option {value === option.value ? 'selected' : ''}"
                    onclick={() => select(option.value)}
                >
                    {option.label}
                    {#if value === option.value}
                        <svg class="check-icon" xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                    {/if}
                </button>
            {/each}
        </div>
    {/if}
</div>

<style lang="scss">
    .custom-select-wrapper {
        position: relative;
        width: 100%;

        .custom-select-trigger {
            display: flex;
            justify-content: space-between;
            align-items: center;
            width: 100%;
            color: var(--dfg-text);
            background: var(--dfg-background-2);
            border: 1px solid transparent;
            border-radius: 5px;
            padding: 10px 14px;
            font-family: "Inter", sans-serif;
            font-size: 14px;
            font-weight: 500;
            cursor: pointer;
            outline: none;
            transition: border-color 0.2s, background 0.2s;

            &:hover {
                background: var(--dfg-background-1);
            }

            &.active {
                border-color: var(--dfg-primary);
            }

            .chevron {
                transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                &.open {
                    transform: rotate(180deg);
                }
            }
        }

        .custom-select-menu {
            position: absolute;
            top: calc(100% + 6px);
            left: 0;
            right: 0;
            background: var(--dfg-background-1);
            border: 1px solid var(--dfg-background-2);
            border-radius: 6px;
            overflow: hidden;
            z-index: 100;
            display: flex;
            flex-direction: column;
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
            animation: slideDown 0.15s ease-out forwards;
            transform-origin: top center;

            .custom-select-option {
                display: flex;
                justify-content: space-between;
                align-items: center;
                width: 100%;
                padding: 12px 14px;
                background: transparent;
                border: none;
                color: var(--dfg-text-faded);
                font-family: "Inter", sans-serif;
                font-size: 14px;
                font-weight: 500;
                cursor: pointer;
                text-align: left;
                transition: background 0.15s, color 0.15s;

                &:hover {
                    background: var(--dfg-background-2);
                    color: var(--dfg-text);
                }

                &.selected {
                    color: var(--dfg-text);
                    font-weight: 600;

                    .check-icon {
                        color: var(--dfg-primary);
                    }
                }
            }
        }
    }

    @keyframes slideDown {
        from {
            opacity: 0;
            transform: scaleY(0.95);
        }
        to {
            opacity: 1;
            transform: scaleY(1);
        }
    }
</style>