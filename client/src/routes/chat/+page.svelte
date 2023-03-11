<script lang="ts">
    import {onMount} from "svelte";
    let status = "🔴";
    let message = "";
    let messages = [];
    let socket: WebSocket;

    onMount(() => {
        socket = new WebSocket("ws://localhost:3000/ws")
        socket.addEventListener("open", () => {
            status = "🟢"
        })

        socket.addEventListener("close", () => {
            status = "🔴";
        })

        socket.addEventListener('message', function (event) {
            messages = [...messages, event.data]
        })
    })

    const sendMessage = () => {
        socket.send(message)
        message = "";
    };



</script>

<h1>Chat room</h1>
<span>Connected: {status}</span>
<div class="messages">
    {#each messages as msg}
    <div class="message">
        {msg}
    </div>
    {/each}
</div>

<form on:submit|preventDefault={sendMessage}>
    <input bind:value={message}>
    <button>Send</button>
</form>

