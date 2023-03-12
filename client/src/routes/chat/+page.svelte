<script lang="ts">
    import {onMount} from "svelte";

    let status = "🔴";
    let statusTip = "Disconnected";
    let message = "";
    let messages = [];
    let socket: WebSocket;

    onMount(() => {
        socket = new WebSocket("ws://localhost:3000/ws")
        socket.addEventListener("open", () => {
            status = "🟢"
            statusTip = "Connected";
        })

        socket.addEventListener("close", () => {
            status = "🔴";
            statusTip = "Disconnected";
        })

        socket.addEventListener('message', function (event) {
            messages = [...messages, event.data]
        })
    })

    const sendMessage = () => {
        socket.send(message)
        message = "";
    };
    const clear_messages = () => {
        messages = [];
    };


</script>
<div class="title flex justify-between">
    <h1 class="text-3xl font-bold cursor-default">Chat Room <span class="tooltip" data-tip="{statusTip}">{status}</span>
    </h1>
    <button class="btn btn-secondary" on:click={clear_messages}>clear</button>
</div>
<div class="card h-96 flex-grow bg-base-300 shadow-xl my-10">
    <div class="card-body">
        <div class="flex flex-col overflow-y-auto max-h-80 scroll-smooth">
            {#each messages as msg}
                <div class="my-2">{msg}</div>
            {/each}
        </div>
    </div>
</div>

<div class="message-box flex justify-end">
    <form on:submit|preventDefault={sendMessage}>
        <input placeholder="Message" class="input input-bordered input-primary w-full sm:w-auto bg-base-200 mb-2"
               bind:value={message}>
        <button class="btn btn-primary w-full sm:w-auto btn-wide">Send</button>
    </form>
</div>
