import { readable } from "svelte/store";
import { getVersion } from "@tauri-apps/api/app";

export const appVersion = readable<string>("", (set) => {
    if (typeof window !== "undefined") {
        getVersion()
            .then((version) => set(version))
            .catch((err) => console.error("Failed to fetch app version:", err));
    }

    return () => {};
});
