<script>
    import "../app.css"
    import {Toaster} from 'svelte-french-toast';
</script>

<Toaster/>
<div class="flex flex-col h-screen sm:justify-center sm:items-center">
    <div class="mx-auto px-3 md:px-0 sm:w-full md:w-4/5 lg:w-3/5 pt-24 sm:pt-0">
        <slot/>
    </div>
</div>