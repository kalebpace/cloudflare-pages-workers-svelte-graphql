<script setup lang="ts">
import { ref, provide } from "vue";

import YouTube from "vue3-youtube";
import VideoSearch from "./components/VideoSearch.vue";
import VideoQueue from "./components/VideoQueue.vue";
import { IVideo } from "./components/VideoQueue.vue";

//https://www.youtube.com/watch?v=9_gkpYORQLU
//https://youtu.be/9_gkpYORQLU
const currentVideo = ref<IVideo>();

function handleSelectedVideo(video: string) {
  console.log(video);
}

function handleQueueUpdated(video: IVideo) {
  if (currentVideo.value !== video) {
    currentVideo.value = video;
  } else {
    return;
  }
}
</script>

<template>
  <div class="max-w-2x">
    <VideoSearch @selected-video="handleSelectedVideo" />
    <YouTube
      class="my-8 rounded-3xl overflow-hidden"
      :src="currentVideo?.id || ''"
    />
    <VideoQueue @queue-updated="handleQueueUpdated" />
  </div>
</template>

<style>
#app {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>
