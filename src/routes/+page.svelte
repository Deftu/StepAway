<script lang="ts">
  import { onMount } from "svelte";
  import { sleep, lock, shutdown, toggleAutoStart } from "$lib/system";
  import { settings, togglePause } from "$lib/settings";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import Dropdown from "$lib/components/Dropdown.svelte";

  onMount(() => {
      settings.load();
  });

  const actionOptions = [
      { value: "sleep", label: "Sleep Displays" },
      { value: "lock", label: "Lock OS" },
      { value: "lock_and_sleep", label: "Lock OS & Sleep " },
      { value: "shutdown", label: "Shutdown System" }
  ];
</script>

<TitleBar />
{#if $settings.isLoaded}
<main class="panel">
  <!-- Quick Actions -->
  <section class="settings-group">
      <h2 class="dfg-title-2">Quick Actions</h2>
      <div class="action-grid">
          <button class="dfg-button action-btn" onclick={sleep}>Sleep</button>
          <button class="dfg-button action-btn" onclick={lock}>Lock</button>
          <button class="dfg-button danger-btn" onclick={shutdown}>Shutdown</button>
      </div>
  </section>

  <hr class="divider" />

  <!-- System Behaviors -->
  <section class="settings-group">
      <div class="row-between">
          <h2 class="dfg-title-2">Launch on Boot</h2>
          <button
              class="dfg-button status-toggle {$settings.isAutostart ? "is-on" : "is-off"}" 
              onclick={() => toggleAutoStart(settings)}
          >
              {$settings.isAutostart ? "Enabled" : "Disabled"}
          </button>
      </div>

      <div class="row-between">
          <h2 class="dfg-title-2">Auto Timer</h2>
          <button
              class="dfg-button status-toggle {$settings.isPaused ? "is-off" : "is-on"}" 
              onclick={() => togglePause(settings)}
          >
              {$settings.isPaused ? "Paused" : "Active"}
          </button>
      </div>

      <div class="control-row">
          <label class="dfg-body-1" for="timeout-slider">
              Idle Timeout: <span class="branding">{$settings.timeoutMinutes} min</span>
          </label>
          <input
              id="timeout-slider"
              type="range"
              min="1"
              max="120"
              bind:value={$settings.timeoutMinutes}
          />
      </div>

      <div class="control-row">
          <label class="dfg-body-1" id="action-label">Action on Timeout</label>
          <Dropdown 
              options={actionOptions} 
              bind:value={$settings.selectedAction} 
              ariaLabelledby="action-label" 
          />
      </div>
  </section>
</main>
{:else}
  <div class="loading-state">
      <span class="dfg-body-1" style="color: var(--dfg-text-faded);">Loading settings...</span>
  </div>
{/if}

<style lang="scss">
  .loading-state {
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100vh;
  }

  .panel {
      display: flex;
      flex-direction: column;
      padding: 10px 24px 24px 24px;
      gap: 24px;
  }

  .divider {
      border: 0;
      height: 1px;
      background: var(--dfg-background-2);
      margin: 0;
  }

  .settings-group {
      display: flex;
      flex-direction: column;
      gap: 16px;

      .row-between {
          display: flex;
          justify-content: space-between;
          align-items: center;
      }

      .action-grid {
          display: grid;
          grid-template-columns: repeat(3, 1fr);
          gap: 10px;
      }

      .action-btn {
          padding: 12px 0;
          font-family: "Inter", sans-serif;
          font-size: 14px;
          font-weight: 600;
          letter-spacing: 0.25px;
      }

      .danger-btn {
          padding: 12px 0;
          font-family: "Inter", sans-serif;
          font-size: 14px;
          font-weight: 600;
          letter-spacing: 0.25px;
          background: var(--dfg-background-2);
          color: var(--dfg-danger);
          transition: all 0.2s ease-in-out;

          &:hover {
              background: var(--dfg-danger);
              color: var(--dfg-text);
          }
      }

      .control-row {
          display: flex;
          flex-direction: column;
          gap: 10px;
      }

      .status-toggle {
          padding: 6px 16px;
          font-family: "Inter", sans-serif;
          font-size: 14px;
          font-weight: 600;
          width: 100px;
          transition: all 0.2s;

          &.is-on {
              color: var(--dfg-on);
              border: 1px solid var(--dfg-on);
          }

          &.is-off {
              color: var(--dfg-error);
              border: 1px solid var(--dfg-error);
          }
      }

      input[type="range"] {
          width: 100%;
          background: transparent;

          &::-webkit-slider-thumb {
              -webkit-appearance: none;
              height: 16px;
              width: 16px;
              border-radius: 50%;
              background: var(--dfg-primary);
              cursor: pointer;
              margin-top: -6px;
          }

          &::-webkit-slider-runnable-track {
              width: 100%;
              height: 4px;
              cursor: pointer;
              background: var(--dfg-background-2);
              border-radius: 2px;
          }
      }
  }
</style>