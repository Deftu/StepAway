<script>
    import "@fontsource/inter";
    import "@fontsource/outfit";
    import "@fontsource/jetbrains-mono";

    import "@fontsource/outfit/600.css";
    import "@fontsource/outfit/700.css";

    import "../app.css";
    import "../theme.css";

    import { onMount } from "svelte";

    onMount(() => {
        // Disable context menu
        window.addEventListener("contextmenu", (e) => {
            e.preventDefault();
        });
    });
</script>

<slot />