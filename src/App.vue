<script lang="ts" setup>
import {onMounted, ref} from "vue";
import {Splitter, SplitterPanel, Toast, useToast} from "primevue";
import {invoke} from "@tauri-apps/api/core";
import {BACKEND_EVENT_GLOBAL_NOTIFICATION, GET_PROJECT_RUNTIME_SUMMARIES_CMD} from "./common.ts";
import {ProjectRuntimeSummary} from "./messages/project.ts";
import GlobalLogArea from "./views/GlobalLogArea.vue";
import {Event, listen} from "@tauri-apps/api/event";
import {GlobalNotificationEvent} from "./messages/notification.ts";

let projectRuntimeSummaries = ref<ProjectRuntimeSummary[]>([]);

onMounted(async () => {
    projectRuntimeSummaries.value = await invoke<ProjectRuntimeSummary[]>(GET_PROJECT_RUNTIME_SUMMARIES_CMD);
});

const toast = useToast();

listen(BACKEND_EVENT_GLOBAL_NOTIFICATION, (event: Event<GlobalNotificationEvent>) => {
    toast.add({
        severity: event.payload.level.toLowerCase(),
        summary: event.payload.summary,
        detail: event.payload.message,
        life: 10000
    })
})

</script>

<template>
    <div class="h-full w-full flex flex-col">
        <div class="flex flex-row h-1/11 justify-start">
            <span class="inline-flex items-center gap-1 px-2 py-2 grow">
                <span class="text-2xl font-black"> RGS<span
                    class="text-primary">PROJECTS</span></span>
            </span>
            <Toast></Toast>
        </div>
        <div class="flex flex-row h-10/11 justify-start grow">
            <ul class="overflow-y-auto h-full w-1/5 border-stone-100 border-r-2" >
                <li style="text-wrap: nowrap">
                    <span
                        class="inline-flex items-center gap-1 px-2 py-2 uppercase text-lg font-black">General</span>
                </li>
                <li style="text-wrap: nowrap">
                    <router-link v-slot="{ href, navigate }" custom to="/github">
                        <a :href="href" class="flex items-center px-4 py-2 cursor-pointer group"
                            @click="navigate">
                            <span class="pi pi-github"/>
                            <span class="ml-2 uppercase text-sm">GITHUB</span>
                        </a>
                    </router-link>
                </li>
                <li style="text-wrap: nowrap">
                    <span
                        class="inline-flex items-center gap-1 px-2 py-2 uppercase text-lg font-black">Projects</span>
                </li>
                <li v-for="project in projectRuntimeSummaries" style="text-wrap: nowrap">
                    <router-link v-slot="{ href, navigate }" :to="'/project/'+project.projectId" custom>
                        <a :href="href" class="flex items-center px-4 py-2 cursor-pointer group"
                            @click="navigate">
                            <span class="pi pi-server"/>
                            <span class="ml-2 uppercase text-sm">{{ project.name }}</span>
                        </a>
                    </router-link>
                </li>
            </ul>
            <div class="w-4/5 h-full flex flex-col">
                <div class="w-full h-7/10 overflow-y-auto pl-16 pr-16 pb-16">
                    <RouterView></RouterView>
                </div>
                <div class="w-full h-3/10 p-4 border-stone-100 border-t-2">
                    <GlobalLogArea></GlobalLogArea>
                </div>
            </div>
            

        </div>
        
    </div>

</template>

<style scoped>

</style>
