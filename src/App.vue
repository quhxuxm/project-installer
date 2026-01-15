<script lang="ts" setup>
import {onMounted, ref} from "vue";
import {Splitter, SplitterPanel} from "primevue";
import {invoke} from "@tauri-apps/api/core";
import {GET_PROJECT_RUNTIME_SUMMARIES_CMD} from "./common.ts";
import {ProjectRuntimeSummary} from "./messages/project.ts";
import LogArea from "./views/LogArea.vue";

let projectRuntimeSummaries = ref<ProjectRuntimeSummary[]>([]);

onMounted(async () => {
    projectRuntimeSummaries.value = await invoke<ProjectRuntimeSummary[]>(GET_PROJECT_RUNTIME_SUMMARIES_CMD);
});
</script>

<template>
    <div class="h-screen w-screen p-3 flex flex-col">
        <div class="flex flex-row justify-start h-1/12">
            <span class="inline-flex items-center gap-1 px-2 py-2 grow">
                <span class="text-2xl font-black"> RGS<span
                    class="text-primary">PROJECTS</span></span>
            </span>
        </div>
        <Splitter class="grow h-11/12" layout="vertical">
            <SplitterPanel :size="80" class="w-full" style="user-select: none">
                <Splitter class="w-full h-full">
                    <SplitterPanel :size="20" class="flex flex-col justify-start pt-4 pl-4">
                        <ul class="overflow-y-auto h-full w-full">
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
                    </SplitterPanel>
                    <SplitterPanel :size="80" class="flex flex-col justify-start">
                        <div class="h-full w-full grow overflow-y-auto p-8">
                            <RouterView></RouterView>
                        </div>
                    </SplitterPanel>
                </Splitter>
            </SplitterPanel>
            <SplitterPanel :size="20" class="flex flex-col h-4/11 w-full" style="user-select: none">
                <LogArea></LogArea>
            </SplitterPanel>
        </Splitter>
    </div>

</template>

<style scoped>

</style>
