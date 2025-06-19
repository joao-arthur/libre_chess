<script lang="ts">
    const { children } = $props();
</script>

<svelte:head>
    <link
        rel="stylesheet"
        href="/css/global.css"
    />
</svelte:head>

{@render children?.()}
