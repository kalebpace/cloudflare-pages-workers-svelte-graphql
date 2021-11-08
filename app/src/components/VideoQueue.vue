<script lang="ts">
export interface IVideo {
  id: string;
  votes: number;
  addedBy?: string;
}
</script>

<script setup lang="ts">
import { ref } from "vue";

let queue = ref<IVideo[]>([]);
const currentSearch = ref("");

function addToQueue(video: IVideo) {
  queue.value.push(video);
}

function removeFromQueue(video: IVideo) {
  queue = ref<IVideo[]>(queue.value.filter((_video) => _video.id === video.id));
}
</script>

<template>
  <div class="flex">
    <div class="bg-gray-400 rounded-xl my-4 w-64" v-for="video in queue">
      <h4>VideoID: {{ video.id }}</h4>
    </div>
  </div>
</template>
