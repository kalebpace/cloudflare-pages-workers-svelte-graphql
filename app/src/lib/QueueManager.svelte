<script lang="ts" context="module">
    export interface IQueueItem {
        id: string
        votes?: number
    }
</script>

<script lang="ts">
    import {createEventDispatcher} from 'svelte';
    const dispatch = createEventDispatcher();

    let _input = ""

    let _queue = []
    $: dispatch('queueUpdated', _queue)

    const queueItemEquals = (a, b) => {
        if(a.votes == b.votes) {
            return 0
        } else if (a.votes < b.votes) {
            return 1
        } else {
            return -1
        }
    }
    function addItem(item: IQueueItem) {
        _queue = [..._queue, item].sort(queueItemEquals)
    }

    function removeItem(item: IQueueItem) {
        _queue = _queue.filter(x => x.id != item.id).sort(queueItemEquals)
    }

    function upvoteItem(item: IQueueItem) {
        const index = _queue.findIndex((x) => x.id == item.id)
        _queue[index].votes += 1
        _queue = _queue.sort(queueItemEquals)
    }

    function _getVideoId(url: string) {
        const urlObj = new URL(url)
        const videoId = urlObj.searchParams.get("v") || urlObj.pathname.slice(1)
        return videoId ? videoId : ""
    }
</script>

<div class="queue-manager">
    <input class="queue-input" bind:value={_input}/>
    <button on:click={() => addItem({id: _getVideoId(_input), votes: 0})}>Add to Queue</button>
    {#each _queue as item (item.id)}
        <div class="queue-item">
            <a href="https://youtube.com/watch?v={item.id}">{item.id}</a>
            <button on:click={() => removeItem({id: item.id})}>Remove Item</button>
            <button on:click={() => upvoteItem({id: item.id})}>Upvote (votes: {item.votes})</button>
        </div>
    {/each}
</div>

<style>
    .queue-manager {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 5%;
    }

    .queue-item {
        display: flex;
        justify-content: space-around;
        align-items: center;
    }
    
    .queue-input {
        width: 640px;
    }
</style>
