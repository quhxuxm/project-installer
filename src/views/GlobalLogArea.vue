<script lang="ts" setup>
import {Event, listen} from "@tauri-apps/api/event";
import {GlobalLogEvent} from "../messages/log.ts";
import {useTemplateRef} from "vue";
import {BACKEND_EVENT_GLOBAL_LOG} from "../common.ts";


let logAreaElement = useTemplateRef<HTMLDivElement>("log-area")

listen(BACKEND_EVENT_GLOBAL_LOG, (event: Event<GlobalLogEvent>) => {
    if (logAreaElement.value){
        let logLineSpan = document.createElement("span")
        logAreaElement.value.appendChild(logLineSpan);
        logLineSpan.classList.add(...["global_log", "log_level_"+event.payload.level.toLowerCase()]);
        logLineSpan.textContent=event.payload.message;
        while(logAreaElement.value.childNodes.length > 2000){
            if (logAreaElement.value.firstChild){
                logAreaElement.value.removeChild(logAreaElement.value.firstChild);
            }
        }
        logAreaElement.value.scrollTop = logAreaElement.value?.scrollHeight;
    }
});

</script>

<template>
    <div class="w-full h-full ">
        <div contenteditable ref="log-area" class="h-full w-full overflow-y-auto text-wrap p-2 text-sm resize-none flex flex-col justify-start border-none outline-none">
        </div>
    </div>
</template>
<style>
.global_log{
    width: 100%;
}
.log_level_info {
    color: #333333; /* Blue for info */
}

.log_level_error {
    color: #ff0000; /* Blue for info */
}

.log_level_warn {
    color: #ff8800; /* Blue for info */
}

.log_level_debug {
    color: #003cff;
}
</style>

