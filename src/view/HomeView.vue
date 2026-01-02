<script setup lang="ts">
import {provide, ref} from "vue";

import Menu from 'primevue/menu';
import {MenuItem} from "primevue/menuitem";
import {ProjectConfiguration, ToolConfiguration} from "../value/configuration.ts";

function initializeToolConfiguration(): ToolConfiguration{
  let toolConfiguration =  new ToolConfiguration();
  let rgsProject=new ProjectConfiguration("rgs", "https://github.com/igt/rgs.git", "D:\\Workspace\\rgs", "6.9-develop");
  rgsProject.description = "RGS Platform"
  toolConfiguration.projects.set("rgs", rgsProject);

  let cptProject =new ProjectConfiguration("cpt", "https://github.com/igt/cpt.git", "D:\\Workspace\\cpt", "6.9-develop");
  cptProject.description="Competition Service"
  cptProject.addDependency(rgsProject);

  toolConfiguration.projects.set("cpt", cptProject);


  toolConfiguration.projects.set("nss", new ProjectConfiguration("nss", "https://github.com/igt/nss.git", "D:\\Workspace\\nss", "6.9-develop"));
  toolConfiguration.projects.set("pas", new ProjectConfiguration("pas", "https://github.com/igt/pas.git", "D:\\Workspace\\pas", "6.9-develop"));
  toolConfiguration.projects.set("gls", new ProjectConfiguration("gls", "https://github.com/igt/gls.git", "D:\\Workspace\\gls", "6.9-develop"));
  toolConfiguration.projects.set("gsr", new ProjectConfiguration("gsr", "https://github.com/igt/gsr.git", "D:\\Workspace\\gsr", "6.9-develop"));
  toolConfiguration.projects.set("cfgs", new ProjectConfiguration("cfgs", "https://github.com/igt/cfgs.git", "D:\\Workspace\\cfgs", "6.9-develop"));
  toolConfiguration.projects.set("cache", new ProjectConfiguration("cache", "https://github.com/igt/cache.git", "D:\\Workspace\\cache", "6.9-develop"));
  toolConfiguration.projects.set("jrs", new ProjectConfiguration("jrs", "https://github.com/igt/jrs.git", "D:\\Workspace\\jrs", "6.9-develop"));
  toolConfiguration.projects.set("gles", new ProjectConfiguration("gles", "https://github.com/igt/gles.git", "D:\\Workspace\\gles", "6.9-develop"));
  toolConfiguration.projects.set("geo", new ProjectConfiguration("geo", "https://github.com/igt/geo.git", "D:\\Workspace\\geo", "6.9-develop"));
  toolConfiguration.projects.set("ruleengine", new ProjectConfiguration("ruleengine", "https://github.com/igt/ruleengine.git", "D:\\Workspace\\ruleengine", "6.9-develop"));
  toolConfiguration.projects.set("flightdeck", new ProjectConfiguration("flightdeck", "https://github.com/igt/flightdeck.git", "D:\\Workspace\\flightdeck", "6.9-develop"));
  toolConfiguration.projects.set("apache", new ProjectConfiguration("apache", "https://github.com/igt/apache.git", "D:\\Workspace\\apache", "6.9-develop"));
 return toolConfiguration;
}

provide("TOOL_CONF", initializeToolConfiguration());

const topLevelMenuItems = ref<MenuItem[]>([
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
        label: 'Node JS',
        icon: 'pi pi-receipt',
        route: '/nodejs'
      },
    ]
  },
  {
    label: 'Projects',
    items: [
      {
        id:'rgs',
        label: 'RGS Platform',
        icon: 'pi pi-server',
        route: '/project/rgs'
      },
      {
        id:'cpt',
        label: 'Competition',
        icon: 'pi pi-server',
        route: '/project/cpt'
      },
      {
        id:'gsr',
        label: 'GSR',
        icon: 'pi pi-server',
        route: '/project/gsr'
      },
      {
        id:'pas',
        label: 'PAS',
        icon: 'pi pi-server',
        route: '/project/pas'
      },
      {
        id:'cfgs',
        label: 'CFGS',
        icon: 'pi pi-server',
        route: '/project/cfgs'
      },
      {
        id:'cache',
        label: 'Cache',
        icon: 'pi pi-server',
        route: '/project/cache'
      },
      {
        id:'nss',
        label: 'NSS',
        icon: 'pi pi-server',
        route: '/project/nss'
      },
      {
        id:'jrs',
        label: 'JRS',
        icon: 'pi pi-server',
        route: '/project/jrs'
      },
      {
        id:'gls',
        label: 'GLS',
        icon: 'pi pi-server',
        route: '/project/gls'
      },
      {
        id:'gles',
        label: 'GLES',
        icon: 'pi pi-server',
        route: '/project/gles'
      },
      {
        id:'geo',
        label: 'GEO',
        icon: 'pi pi-server',
        route: '/project/geo'
      },
      {
        id:'ruleengine',
        label: 'Rule Engine',
        icon: 'pi pi-server',
        route: '/project/ruleengine'
      },
      {
        id:'flightdeck',
        label: 'FlightDeck',
        icon: 'pi pi-server',
        route: '/project/flightdeck'
      },
      {
        id:'apache',
        label: 'Apache',
        icon: 'pi pi-server',
        route: '/project/apache'
      },
    ]
  }
]);

</script>

<template>
  <div class="h-screen w-screen flex flex-row space-x-4 p-4">

    <Menu :model="topLevelMenuItems" class="flex flex-col justify-center h-full w-full md:w-80 overflow-y-auto px-4" >
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
          <a :href="href"  @click="navigate" class="flex items-center px-4 py-2 cursor-pointer group">
            <span :class="subMenuItem.icon" />
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
