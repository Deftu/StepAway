import { get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { enable, disable } from "@tauri-apps/plugin-autostart";
import type { SettingsStore } from "$lib/settings";

export function sleep() {
    invoke("sleep");
}

export function lock() {
    invoke("lock");
}

export function lockAndSleep() {
    invoke("lock_and_sleep");
}

export function shutdown() {
    invoke("shutdown");
}

export async function toggleAutoStart(store: SettingsStore) {
    try {
        const settings = get(store);
        if (settings.isAutostart) {
            await disable();
            settings.isAutostart = false;
        } else {
            await enable();
            settings.isAutostart = true;
        }

        store.set(settings);
    } catch (err) {
        console.error("Failed to toggle autostart:", err);
    }
}
