<script lang="ts" setup>
import {Event, listen} from "@tauri-apps/api/event";
import {LogEvent} from "../messages/log.ts";
import {ref} from "vue";

let logEvents = ref<LogEvent[]>([]);

listen("log-event", (event: Event<LogEvent>) => {
  logEvents.value.push(event.payload)
})

</script>

<style scoped>

</style>

<template>
  <div class="w-full h-full pt-4 pb-4 overflow-y-auto">
    <ul class="w-full flex flex-col gap-2">
      <li v-for="log of logEvents">
        {{ log.source }}-{{ log.level }}-{{ log.date }}-{{ log.message }}
      </li>
    </ul>
  </div>
</template>

