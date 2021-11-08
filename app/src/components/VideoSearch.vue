<script setup lang="ts">
import { ref } from "vue";
import { SearchIcon } from "@heroicons/vue/outline";

const emit = defineEmits<{
  (e: "selected-video", videoId: string): void;
}>();

const currentSearch = ref("");

function handleSearchInput(_event: KeyboardEvent) {
  const videoId = getYoutubeId(currentSearch.value);
  emit("selected-video", videoId);
}

function getYoutubeId(url: string) {
  const longYTUrl = /youtube\.com/.test(url);
  const shortYTUrl = /youtu\.be/.test(url);

  let videoId: string | null;
  if (longYTUrl) {
    videoId = new URL(url).searchParams.get("v");
  } else if (shortYTUrl) {
    videoId = new URL(url).pathname.split("/")[1];
  } else {
    throw new Error("Could not match URL format");
  }
  if (videoId) {
    return videoId as string;
  } else {
    throw new Error("Video ID could not be found");
  }
}
</script>

<template>
  <div
    class="
      inline-flex
      rounded-xl
      my-4
      border-solid border-2 border-grey
      p-1
      w-full
    "
  >
    <SearchIcon class="m-2 h-5 w-5 text-grey-500" />
    <input
      class="ml-2 w-full outline-none"
      type="text"
      v-model="currentSearch"
      placeholder="Search youtube..."
      @keyup.enter="handleSearchInput"
    />
  </div>
</template>
