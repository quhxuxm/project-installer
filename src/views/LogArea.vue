<script lang="ts" setup>
import {Event, listen} from "@tauri-apps/api/event";
import {Column, DataTable} from "primevue";
import {LogEvent} from "../messages/log.ts";
import {ref} from "vue";
import {BACKEND_EVENT_LOG_MESSAGE} from "../common.ts";

let logEvents = ref<LogEvent[]>([]);

listen(BACKEND_EVENT_LOG_MESSAGE, (event: Event<LogEvent>) => {
  logEvents.value.push(event.payload);
  while (logEvents.value.length > 100) {
    logEvents.value.shift();
  }
})

</script>

<style scoped>

</style>

<template>
  <div class="w-full h-full">
    <DataTable :value="logEvents" column-resize-mode="fit" resizableColumns scrollHeight="400px" scrollable
               showGridlines
               stripedRows
               table-style="min-width: 50rem">
      <Column class="w-1/7" field="projectId" header="PROJECT" header-class="text-sm">
        <template #body="slot">
          <span :class="slot.data.level.toLowerCase()" class="text-wrap w-full text-xs uppercase">
            {{ slot.data.projectId }}
          </span>
        </template>
      </Column>
      <Column class="w-1/7" field="level" header="LEVEL" header-class="text-sm">
        <template #body="slot">
          <span :class="slot.data.level.toLowerCase()" class="text-wrap w-full text-xs uppercase">
            {{ slot.data.level }}
          </span>
        </template>
      </Column>
      <Column class="w-5/7" field="message" header="MESSAGE" header-class="text-sm">
        <template #body="slot">
          <span :class="slot.data.level.toLowerCase()" class="text-wrap w-full text-xs">
            {{ slot.data.message }}
          </span>
        </template>
        >
      </Column>
    </DataTable>
  </div>
</template>
<style scoped>
.info {
  color: #029310; /* Blue for info */
}

.error {
  color: #ff0000; /* Blue for info */
}

.warn {
  color: #ff8800; /* Blue for info */
}

.debug {
  color: #003cff;
}
</style>

