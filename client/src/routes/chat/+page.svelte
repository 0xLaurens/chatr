<script lang="ts">
    import {onMount, onDestroy} from "svelte";
    import {user, channel} from "../../lib/stores/user";
    import {goto} from '$app/navigation';
    import {env} from '$env/dynamic/public'
    import toast from "svelte-french-toast";

    let status = "🔴";
    let statusTip = "Disconnected";
    let message = "";
    let messages = [];
    let socket: WebSocket;
    let interval: number;
    let delay = 2000;
    let timeout = false;
    $: {
        if (interval || (!timeout && interval)) {
            clearInterval(interval);
        }

        if (timeout == true) {
            interval = setInterval(() => {
                if (delay < 30_000) delay = delay * 2;
                console.log("reconnecting in:", delay)
                connect();
            }, delay)
        }
    }

    function connect() {
        socket = new WebSocket(`${env.PUBLIC_WEBSOCKET_URL}/ws`)
        socket.addEventListener("open", () => {
            status = "🟢"
            statusTip = "Connected";
            timeout = false;
            socket.send(`{"username": "${$user}", "channel": "${$channel}"}`);
        })

        socket.addEventListener("close", () => {
            status = "🔴";
            statusTip = "Disconnected";
            if (timeout == false) {
                delay = 2000;
                timeout = true;
            }
        })

        socket.addEventListener('message', function (event) {
            if (event.data == "Username already taken.") {
                toast.error(event.data)
                goto("/");
            } else {
                messages = [...messages, event.data]
            }
        })
    }

    onMount(() => {
            if ($user.length < 1 || $channel.length < 1) {
                toast.error("Something went wrong!")
                goto("/");
            } else {
                connect()
            }
        }
    )

    onDestroy(() => {
        if (socket) {
            socket.close()
        }
        if (interval) {
            clearInterval(interval)
        }
        timeout = false
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
    <button class="btn btn-accent" on:click={clear_messages}>clear</button>
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
