<script lang="ts">
  import YoutubePlayer from './lib/YoutubePlayer.svelte'
  import QueueManager from './lib/QueueManager.svelte';
  import type {IQueueItem} from './lib/QueueManager.svelte';

  let currentVideo: IQueueItem = { id: "trinU3VD1Zo" };
</script>

<main>
  <YoutubePlayer bind:videoId={currentVideo.id} />
  <QueueManager on:queueUpdated={(event) => currentVideo = event.detail[0]}/>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  main {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: 1em;
    margin: 0 auto;
  }
</style>
