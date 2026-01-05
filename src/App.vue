<script setup>
import {ref} from "vue";

import Menu from 'primevue/menu';
import {invoke} from "@tauri-apps/api/core";
import {BackendCommandKeys} from "./common.js";

let menuItemsRef = ref();

async function initMenuItems() {
  let projectMenuItems = [];
  let globalState = await invoke(BackendCommandKeys.LOAD_GLOBAL_STATE);
  for (const [key, value] of globalState.projects) {
    let item = {
      id: key,
      label: value.name,
      icon: 'pi pi-server',
      route: `/project/${key}`
    };
    projectMenuItems.push(item);
  }

  menuItemsRef.value = [
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
}

initMenuItems().then((items) => {
  menuItemsRef.value = items;
});


</script>

<template>
  <div class="h-screen w-screen flex flex-row space-x-4 p-4">

    <Menu :model="menuItemsRef" class="flex flex-col justify-center h-full w-full md:w-80 overflow-y-auto px-4">
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
