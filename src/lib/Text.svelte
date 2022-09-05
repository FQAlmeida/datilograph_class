<script lang="ts">
    export let paragraph: string = "";
    export let next: number;
    export let wrong: boolean;
</script>

<h3>Paragraph</h3>
<span class="right">
    {paragraph.substring(0, next)}
</span>
<span class={wrong ? "wrong" : "try"}>
    {paragraph[next]}
</span>
<span class="notdone">
    {paragraph.substring(next + 1)}
</span>

<style>
    .wrong {
        background-color: red;
    }
    .try {
        background-color: orange;
    }
    .right {
        background-color: green;
    }
    .notdone {
        background-color: transparent;
    }
</style>
