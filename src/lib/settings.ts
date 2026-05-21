import { writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { isEnabled } from "@tauri-apps/plugin-autostart";

export interface AppSettings {
    timeoutMinutes: number;
    selectedAction: string;
    isPaused: boolean;
    isAutostart: boolean;
    isLoaded: boolean;
}

const defaultSettings: AppSettings = {
    timeoutMinutes: 5,
    selectedAction: "sleep",
    isPaused: false,
    isAutostart: false,
    isLoaded: false,
};

export interface SettingsStore extends Writable<AppSettings> {
    load: () => Promise<void>;
}

function createSettingsStore(): SettingsStore {
    const { subscribe, set, update } = writable<AppSettings>(defaultSettings);

    const syncWithRust = (state: AppSettings) => {
        if (!state.isLoaded || typeof window === "undefined") return;

        invoke("update_settings", {
            timeout: state.timeoutMinutes,
            action: state.selectedAction,
            paused: state.isPaused,
        }).catch((err) => console.error("Failed to sync settings:", err));
    };

    return {
        subscribe,
        set: (value: AppSettings) => {
            set(value);
            syncWithRust(value);
        },
        update: (updater: (value: AppSettings) => AppSettings) => {
            update((currentValue) => {
                const newValue = updater(currentValue);
                syncWithRust(newValue);
                return newValue;
            });
        },
        load: async () => {
            try {
                const savedState = await invoke<{
                    timeout: number;
                    action: string;
                    paused: boolean;
                }>("get_settings");
                const autostartActive = await isEnabled();

                set({
                    timeoutMinutes: savedState.timeout,
                    selectedAction: savedState.action,
                    isPaused: savedState.paused,
                    isAutostart: autostartActive,
                    isLoaded: true,
                });
            } catch (err) {
                console.error("Failed to load settings.", err);
                update((s) => ({ ...s, isLoaded: true }));
            }
        },
    };
}

export function togglePause(store: SettingsStore) {
    store.update((s) => {
        s.isPaused = !s.isPaused;
        return s;
    });
}

export const settings = createSettingsStore();
