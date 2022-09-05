<script lang="ts">
    export let onKeyPressed: (keyValue: string) => void;
    const keyPressedHandler = (ev: KeyboardEvent) => {
        onKeyPressed(ev.key)
        console.log(ev);
    };
</script>

<svelte:window on:keypress|preventDefault={keyPressedHandler} />
