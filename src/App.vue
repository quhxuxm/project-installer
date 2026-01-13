<script lang="ts" setup>
import {onMounted, ref} from "vue";
import {Menu, ScrollPanel, Splitter, SplitterPanel} from "primevue";
import {MenuItem} from "primevue/menuitem";
import {invoke} from "@tauri-apps/api/core";
import {GET_PROJECT_RUNTIME_SUMMARIES_CMD} from "./common.ts";
import {ProjectRuntimeSummary} from "./messages/project.ts";

let menuItems = ref<MenuItem[]>([]);

onMounted(async () => {
  let projectRuntimeSummaries = await invoke<ProjectRuntimeSummary[]>(
      GET_PROJECT_RUNTIME_SUMMARIES_CMD,
  );
  let projectMenuItems = new Array<MenuItem>();

  projectRuntimeSummaries.forEach((projectRuntimeSummary) => {
    let item = {
      id: projectRuntimeSummary.projectId,
      label: projectRuntimeSummary.name,
      icon: "pi pi-server",
      route: `/project/${projectRuntimeSummary.projectId}`,
    };
    projectMenuItems.push(item);
  });
  console.log(projectMenuItems);
  menuItems.value = [
    {
      label: "General",
      items: [
        {
          label: "GitHub",
          icon: "pi pi-github",
          route: "/github",
        },
        {
          label: "Java",
          icon: "pi pi-android",
          route: "/java",
        },
        {
          label: "Maven",
          icon: "pi pi-wallet",
          route: "/maven",
        },
        {
          label: "Kafka",
          icon: "pi pi-shop",
          route: "/kafka",
        },
        {
          label: "Node JS",
          icon: "pi pi-receipt",
          route: "/nodejs",
        },
      ],
    },
    {
      label: "Projects",
      items: projectMenuItems,
    },
  ];
});
</script>

<template>


  <Splitter class="h-screen w-screen" layout="vertical">
    <SplitterPanel class="w-full h-3/5 gap-4 p-4">
      <div class="h-full w-full">
        <div class="h-full w-full flex flex-row gap-4">
          <Menu
              :model="menuItems"
              class="flex flex-col justify-start w-1/3 h-full md:w-70 overflow-y-auto px-3"
          >
            <template #start>
                <span class="inline-flex items-center gap-1 px-2 py-2">
                    <span class="text-xl font-black">
                        RGS<span class="text-primary">PROJECTS</span>
                    </span>
                </span>
            </template>
            <template #submenuheader="{ item: subMenuItem }">
                <span class="text-primary uppercase">{{
                    subMenuItem.label
                  }}</span>
            </template>
            <template #item="{ item: subMenuItem }">
              <router-link
                  v-if="subMenuItem.route"
                  v-slot="{ href, navigate }"
                  :to="subMenuItem.route"
                  custom
              >
                <a
                    :href="href"
                    class="flex items-center px-4 py-2 cursor-pointer group"
                    @click="navigate"
                >
                  <span :class="subMenuItem.icon"/>
                  <span class="ml-2 uppercase text-sm">{{
                      subMenuItem.label
                    }}</span>
                </a>
              </router-link>
            </template>
          </Menu>
          <ScrollPanel class="h-full grow">
            <RouterView></RouterView>
          </ScrollPanel>
        </div>

      </div>
    </SplitterPanel>
    <SplitterPanel class="flex flex-col h-2/5 w-full">
      <div class="w-full h-full p-5 bg-amber-400"></div>
    </SplitterPanel>
  </Splitter>


</template>

<style scoped></style>
