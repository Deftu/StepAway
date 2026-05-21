import { invoke } from "@tauri-apps/api/core";

export function sleep() {
    invoke("sleep");
}

export function lock() {
    invoke("lock");
}

export function shutdown() {
    invoke("shutdown");
}
