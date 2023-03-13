<script lang="ts">
    import {user} from "$lib/stores/user"
    import { goto } from '$app/navigation';

    let username = "";
    const join_room = () => {
        if(username.length < 1) {
            username = "user" + Math.floor(Math.random() * 1000);
        }

        user.set(username)
        goto("/chat");
    }
</script>

<div class="flex flex-col justify-center">
    <div class="title">
        <h1 class="text-3xl font-bold text-center">Chatr: a Websocket chatroom</h1>
    </div>
    <div class="join self-center my-5">
        <input placeholder="Username" bind:value={username}
               class="input input-bordered input-primary w-full sm:w-auto bg-base-200 mb-2 mr-3">
        <button class="btn btn-primary" on:click={join_room}>Join Room.</button>
    </div>
    <div class="github self-center">
        <p>
            Check out <a class="link link-accent" href="https://github.com/0xLaurens/chatr" target="_blank"
                         rel="noreferrer">Chatr</a>, to view the source code!
        </p>
    </div>

</div>
