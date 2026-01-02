<script setup lang="ts">
import {useRoute} from "vue-router";
import {inject, ref, watch} from "vue";
import {ProjectConfiguration, ToolConfiguration} from "../value/configuration.ts";
import ScrollPanel from "primevue/scrollpanel";
import {DataTable} from "primevue";
import Fieldset from "primevue/fieldset";
import {InputText} from "primevue";
import {Button} from "primevue";
import Column from 'primevue/column';

let currentRoute = useRoute();

let toolConfiguration=inject<ToolConfiguration>("TOOL_CONF");

let currentProject= ref<ProjectConfiguration | undefined>(toolConfiguration?.projects.get(currentRoute.params.id as string));

watch(()=>currentRoute.params.id as string, (newProjectId, _)=>{
  currentProject.value=toolConfiguration?.projects.get(newProjectId)
});

</script>

<style scoped>

</style>

<template>

  <ScrollPanel class="h-full px-4">
    <h1 class="text-2xl text-primary mb-4">{{currentProject?.description ?? '' }}</h1>
    <div class="flex flex-col gap-4">
      <Fieldset legend="Repository configuration" class="text-xl"  toggleable>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-2">
            <label for="githubBranch" class="text-lg">Branch</label>
            <InputText id="githubBranch" />
            <Message class="text-gray-500 text-sm" size="small" severity="secondary" variant="simple">Enter the GitHub branch.</Message>
          </div>
          <div class="flex flex-col gap-2">
            <label for="githubRepositoryUrl" class="text-lg">Repository url</label>
            <InputText id="githubRepositoryUrl" />
            <Message class="text-gray-500 text-sm" size="small" severity="secondary" variant="simple">Enter the GitHub repository url.</Message>
          </div>
          <div class="flex flex-col gap-2">
            <label for="localRepoPath" class="text-lg">Local repository path</label>
            <InputText id="localRepoPath" />
            <Message class="text-gray-500 text-sm" size="small" severity="secondary" variant="simple">Enter the local repository path.</Message>
          </div>
        </div>

      </Fieldset>
      <Fieldset legend="Command configuration" class="text-xl"  toggleable>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-2 mb-4">
            <label for="buildCommand" class="text-lg">Build command</label>
            <InputText id="buildCommand" />
            <Message class="text-gray-500 text-sm" size="small" severity="secondary" variant="simple">Enter the build command.</Message>
          </div>
          <div class="flex flex-col gap-2 mb-4">
            <label for="runCommand" class="text-lg">Run command</label>
            <InputText id="runCommand" />
            <Message class="text-gray-500 text-sm" size="small" severity="secondary" variant="simple">Enter the run command.</Message>
          </div>
          <div class="flex flex-col gap-2">
            <label for="debugCommand" class="text-lg">Debug command</label>
            <InputText id="debugCommand" />
            <Message class="text-gray-500 text-sm" size="small" severity="secondary" variant="simple">Enter the debug command.</Message>
          </div>
        </div>
      </Fieldset>
      <Fieldset legend="Startup dependencies" class="text-xl" toggleable v-if="currentProject && currentProject.dependencies.length>0">
        <DataTable :value="currentProject.dependencies" class="text-base">
          <Column field="id" header="Id" class="uppercase">
            <template #body="slotProps">
              <router-link v-if="slotProps.data.id" v-slot="{ href, navigate }" :to="'/project/'+slotProps.data.id" custom>
                <a :href="href"  @click="navigate">
                  <span>{{ slotProps.data.id }}</span>
                </a>
              </router-link>
            </template>
          </Column>
          <Column field="description" header="Description" class="uppercase">
            <template #body="slotProps">
              <router-link v-if="slotProps.data.id" v-slot="{ href, navigate }" :to="'/project/'+slotProps.data.id" custom>
                <a :href="href"  @click="navigate">
                  <span>{{ slotProps.data.description }}</span>
                </a>
              </router-link>
            </template>
          </Column>
          <Column field="status" header="Running status" class="uppercase"></Column>
        </DataTable>
      </Fieldset>
      <div class="flex flex-row gap-4 m-4 justify-end" >
        <Button class="uppercase">Pull code</Button>
        <Button class="uppercase">Build</Button>
        <Button class="uppercase">Run</Button>
        <Button class="uppercase">Debug</Button>
      </div>
    </div>
  </ScrollPanel>
</template>

