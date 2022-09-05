<script lang="ts">
    import InputDisplay from "./InputDisplay.svelte";
    import Keyboard from "./Keyboard.svelte";
    import Text from "./Text.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    let paragraph: string = "";

    onMount(async () => {
        let text: string = await invoke("lorem", { words: 32 });
        paragraph = text;
    });

    let input: string = "";

    const onKeyPressed = (keyValue: string) => {
        if (paragraph[next] != keyValue) {
            wrong = true;
            return;
        }
        wrong = false;
        next++;
        if(keyValue == " "){
            keyValue = "*"
        }
        input += keyValue;
    };

    let next = 0;
    let wrong = false;
</script>

<Text {paragraph} {next} {wrong} />
<hr />
<InputDisplay {input} />
<hr />
<Keyboard {onKeyPressed} />
