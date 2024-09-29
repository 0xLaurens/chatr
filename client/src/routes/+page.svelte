<script lang="ts">
    import {user, channel} from "$lib/stores/user"
    import {goto, invalidate} from '$app/navigation';
    import { env } from '$env/dynamic/public'
    import toast from 'svelte-french-toast';

    let status, rooms;
    export let data;
    $:({status, rooms} = data);

    let username = "";
    let room = "";
    const join_room = () => {
        user.set(username);
        channel.set(room);
        goto("/chat");
    }
    const select_room = (selected_room: String) => {
        room = selected_room;
    };
    const filled_in = () => {
        return !(username.length > 0 && room.length > 0);
    };

    const reload = () => {
        toast.success("Reloaded rooms")
        let url = `${env.PUBLIC_API_URL}`;
        if (url.endsWith("/")) {
            url = url.slice(0, -1);
        }
        invalidate(`${url}/rooms`);
    }
</script>

<div class="flex flex-col justify-center">
    <div class="title">
        <h1 class="text-3xl font-bold text-center">Chatr: a Websocket chatroom</h1>
    </div>
    <div class="join self-center">
    </div>
    <div class="rooms self-center my-5">
        <div class="flex justify-between py-2">
            <h2 class="text-xl font-bold ">
                List of active chatroom's
            </h2>
            <button class="btn btn-square btn-sm btn-accent" on:click={reload}>↻</button>
        </div>
        {#if status && rooms.length < 1}
            <div class="card bg-base-300 w-96 shadow-xl text-center">
                <div class="card-body">
                    <h3 class="card-title ">{status}</h3>
                </div>
            </div>
        {/if}
        {#if rooms}
            {#each rooms as room}
                <div class="card bg-base-300 w-96 shadow-xl my-3" on:click={select_room(room)}>
                    <div class="card-body">
                        <div class="flex justify-between">
                            <h2 class="card-title">{room}</h2>
                            <button class="btn btn-primary btn-md">Select Room</button>
                        </div>
                    </div>
                </div>
            {/each}
        {/if}
    </div>
    <div class="create self-center my-5">
        <div>
            <label class="label">
                <span class="label-text">Username</span>
            </label>
            <input placeholder="Username" bind:value={username}
                   class="input input-bordered input-primary w-full bg-base-200 mb-4 mr-3">
        </div>
        <label class="label">
            <span class="label-text">Room name</span>
        </label>
        <input placeholder="Room Name" bind:value={room}
               class="input input-bordered input-primary w-full sm:w-auto bg-base-200 mb-2 mr-3">
        <button class="btn btn-primary" disabled="{filled_in(username, room)}" on:click={join_room}>Join Room.</button>
    </div>
    <div class="github self-center">
        <p>
            Check out <a class="link link-accent" href="https://github.com/0xLaurens/chatr" target="_blank"
                         rel="noreferrer">Chatr</a>, to view the source code!
        </p>
    </div>
</div>
