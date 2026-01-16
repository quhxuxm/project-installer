<script lang="ts" setup>
import {Event, listen} from "@tauri-apps/api/event";
import {Column, DataTable} from "primevue";
import {GlobalLogEvent} from "../messages/log.ts";
import {ref} from "vue";
import {BACKEND_EVENT_GLOBAL_LOG} from "../common.ts";

let logEvents = ref<GlobalLogEvent[]>([]);

listen(BACKEND_EVENT_GLOBAL_LOG, (event: Event<GlobalLogEvent>) => {
    logEvents.value.push(event.payload);
    while (logEvents.value.length > 1000) {
        logEvents.value.shift();
    }
})

</script>

<style scoped>

</style>

<template>
    <div class="w-full h-full ">
        <DataTable :value="logEvents" class="h-full w-full" resizableColumns
                   scrollHeight="100%"
                   scrollable
                   showGridlines
                   stripedRows>
            <Column class="w-1/11" field="projectId" header="PROJECT" header-class="text-sm">
                <template #body="slot">
                  <span :class="slot.data.level.toLowerCase()" class="text-wrap w-full text-xs uppercase"
                        style="user-select: auto">
                    {{ slot.data.projectId }}
                  </span>
                </template>
            </Column>
            <!--            <Column class="w-1/11" field="level" header="LOG LEVEL" header-class="text-sm">-->
            <!--                <template #body="slot">-->
            <!--                  <span :class="slot.data.level.toLowerCase()" class="text-wrap w-full text-xs uppercase"-->
            <!--                        style="user-select: auto">-->
            <!--                    {{ slot.data.level }}-->
            <!--                  </span>-->
            <!--                </template>-->
            <!--            </Column>-->
            <Column class="w-10/11" field="message" header="MESSAGE" header-class="text-sm">
                <template #body="slot">
                  <span :class="slot.data.level.toLowerCase()" class="text-wrap w-full text-xs"
                        style="user-select: auto">
                    {{ slot.data.message }}
                  </span>
                </template>
            </Column>
        </DataTable>
    </div>
</template>
<style scoped>
.info {
    color: #333333; /* Blue for info */
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

