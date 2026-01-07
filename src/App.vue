<script lang="ts" setup>
import {onMounted, provide, ref} from "vue";
import Menu from 'primevue/menu';
import {MenuItem} from "primevue/menuitem";
import {invoke} from "@tauri-apps/api/core";
import {ApplicationStateOutput} from "./messages/application_state_output.ts";
import {APPLICATION_STATE, LOAD_APPLICATION_STATE_CMD} from "./common.ts";


let applicationState = ref<ApplicationStateOutput>();
provide(APPLICATION_STATE, applicationState);

let menuItems = ref<MenuItem[]>();

onMounted(async () => {
  applicationState.value = await invoke<ApplicationStateOutput>(LOAD_APPLICATION_STATE_CMD);
  let projectMenuItems = new Array<MenuItem>();
  let projectsObj = applicationState.value?.projects ?? {};
  Object.keys(projectsObj).forEach((projectId) => {
    let item = {
      id: projectId,
      label: projectsObj[projectId].name ?? projectId,
      icon: 'pi pi-server',
      route: `/project/${projectId}`
    };
    projectMenuItems.push(item);
  });
  menuItems.value = [
    {
      label: 'General',
      items: [
        {
          label: 'GitHub',
          icon: 'pi pi-github',
          route: '/github'
        },
        {
          label: 'Java',
          icon: 'pi pi-android',
          route: '/java'
        },
        {
          label: 'Maven',
          icon: 'pi pi-wallet',
          route: '/maven'
        },
        {
          label: 'Kafka',
          icon: 'pi pi-shop',
          route: '/kafka'
        },
        {
          label: 'Node JS',
          icon: 'pi pi-receipt',
          route: '/nodejs'
        },
      ]
    },
    {
      label: 'Projects',
      items: projectMenuItems
    }
  ];
});

</script>

<template>
  <div class="h-screen w-screen flex flex-row space-x-4 p-4">
    <Menu :model="menuItems"
          class="flex flex-col justify-start h-full w-full md:w-70 overflow-y-auto px-3">
      <template #start>
                <span class="inline-flex items-center gap-1 px-2 py-2">
                    <span class="text-xl font-black">
                      RGS<span class="text-primary">PROJECTS</span>
                    </span>
                </span>
      </template>
      <template #submenuheader="{ item: subMenuItem }">
        <span class="text-primary uppercase ">{{ subMenuItem.label }}</span>
      </template>
      <template #item="{ item: subMenuItem }">
        <router-link v-if="subMenuItem.route" v-slot="{ href, navigate }" :to="subMenuItem.route" custom>
          <a :href="href" class="flex items-center px-4 py-2 cursor-pointer group" @click="navigate">
            <span :class="subMenuItem.icon"/>
            <span class="ml-2 uppercase text-sm">{{ subMenuItem.label }}</span>
          </a>
        </router-link>
      </template>
    </Menu>

    <Panel class="flex flex-col grow h-full">
      <RouterView></RouterView>
    </Panel>

  </div>
</template>

<style scoped>

</style>
