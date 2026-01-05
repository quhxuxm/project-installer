<script lang="ts" setup>
import {useRoute} from "vue-router";
import {inject, ref, watch} from "vue";
import ScrollPanel from "primevue/scrollpanel";
import {Button, DataTable, InputText} from "primevue";
import Fieldset from "primevue/fieldset";
import Column from 'primevue/column';
import {APPLICATION_STATE} from "../common.ts";
import {ProjectState} from "../messages/project_state.ts";

let currentRoute = useRoute();

let applicationState = inject(APPLICATION_STATE);

let currentProject = ref<ProjectState | undefined>(applicationState?.value?.projects[currentRoute.params.id as string]);

watch(() => currentRoute.params.id as string, (newProjectId, _) => {
  currentProject.value = applicationState?.value?.projects[newProjectId];
});

</script>

<style scoped>

</style>

<template>

  <ScrollPanel class="h-full px-4">
    <h1 class="text-2xl text-primary mb-4">{{ currentProject?.name ?? '' }}</h1>
    <div class="flex flex-col gap-4">
      <Fieldset class="text-xl" legend="Repository configuration" toggleable>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="githubBranch">Branch</label>
            <InputText id="githubBranch"/>
            <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
              the GitHub
              branch.
            </Message>
          </div>
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="githubRepositoryUrl">Repository url</label>
            <InputText id="githubRepositoryUrl"/>
            <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
              the GitHub
              repository url.
            </Message>
          </div>
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="localRepoPath">Local repository path</label>
            <InputText id="localRepoPath"/>
            <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
              the local
              repository path.
            </Message>
          </div>
        </div>

      </Fieldset>
      <Fieldset class="text-xl" legend="Command configuration" toggleable>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-2 mb-4">
            <label class="text-lg" for="buildCommand">Build command</label>
            <InputText id="buildCommand"/>
            <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
              the build
              command.
            </Message>
          </div>
          <div class="flex flex-col gap-2 mb-4">
            <label class="text-lg" for="runCommand">Run command</label>
            <InputText id="runCommand"/>
            <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
              the run
              command.
            </Message>
          </div>
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="debugCommand">Debug command</label>
            <InputText id="debugCommand"/>
            <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
              the debug
              command.
            </Message>
          </div>
        </div>
      </Fieldset>
      <Fieldset v-if="currentProject && currentProject.startupDependencies.length>0" class="text-xl"
                legend="Startup dependencies"
                toggleable>
        <DataTable :value="currentProject.startupDependencies" class="text-base">
          <Column class="uppercase" field="id" header="Id">
            <template #body="slotProps">
              <router-link v-if="slotProps.data.id" v-slot="{ href, navigate }"
                           :to="'/project/'+slotProps.data.id"
                           custom>
                <a :href="href" @click="navigate">
                  <span>{{ slotProps.data.id }}</span>
                </a>
              </router-link>
            </template>
          </Column>
          <Column class="uppercase" field="description" header="Description">
            <template #body="slotProps">
              <router-link v-if="slotProps.data.id" v-slot="{ href, navigate }"
                           :to="'/project/'+slotProps.data.id"
                           custom>
                <a :href="href" @click="navigate">
                  <span>{{ slotProps.data.description }}</span>
                </a>
              </router-link>
            </template>
          </Column>
          <Column class="uppercase" field="status" header="Running status"></Column>
        </DataTable>
      </Fieldset>
      <div class="flex flex-row gap-4 m-4 justify-end">
        <Button class="uppercase">Pull code</Button>
        <Button class="uppercase">Build</Button>
        <Button class="uppercase">Run</Button>
        <Button class="uppercase">Debug</Button>
      </div>
    </div>
  </ScrollPanel>
</template>

