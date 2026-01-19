<script lang="ts" setup>
import {Event, listen} from "@tauri-apps/api/event";
import {GlobalLogEvent} from "../messages/log.ts";
import {useTemplateRef} from "vue";
import {BACKEND_EVENT_GLOBAL_LOG} from "../common.ts";


let logAreaElement = useTemplateRef<HTMLDivElement>("log-area")

listen(BACKEND_EVENT_GLOBAL_LOG, (event: Event<GlobalLogEvent>) => {
    if (logAreaElement.value){
        let logLineSpan = document.createElement("span")
        logLineSpan.className =  event.payload.level.toLowerCase();
        logLineSpan.textContent=event.payload.message;
        logAreaElement.value.appendChild(logLineSpan);
        while(logAreaElement.value.childNodes.length > 2000){
            if (logAreaElement.value.firstChild){
                logAreaElement.value.removeChild(logAreaElement.value.firstChild);
            }
            
        }
        logAreaElement.value.scrollTop = logAreaElement.value?.scrollHeight;
    }
});

</script>

<style scoped>

</style>

<template>
    <div class="w-full h-full ">
        <div contenteditable ref="log-area" class="h-full w-full overflow-y-auto text-wrap p-2 text-sm resize-none flex flex-col justify-start border-none outline-none">
        </div>
    </div>
</template>
<style scoped>
.log{
    width: 100%;
}
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

