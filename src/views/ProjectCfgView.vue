<script lang="ts" setup>
import {useRoute} from "vue-router";
import {inject, ref, watch} from "vue";
import ScrollPanel from "primevue/scrollpanel";
import {Button, DataTable, InputText, Select} from "primevue";
import Fieldset from "primevue/fieldset";
import Column from 'primevue/column';
import {APPLICATION_STATE} from "../common.ts";
import {ProjectState} from "../messages/project_state.ts";

let currentRoute = useRoute();

let applicationState = inject(APPLICATION_STATE);

let currentProject = ref<ProjectState | undefined>(applicationState?.value?.projects[currentRoute.params.id as string]);


let githubBranch = ref(currentProject?.value?.configuredGithubBranch);
let githubRepoUrl = ref(currentProject?.value?.githubRepoUrl);
let localRepoPath = ref(currentProject?.value?.localRepoPath);
let buildCommand = ref(`${currentProject?.value?.buildCommand?.cmd} ${currentProject?.value?.buildCommand?.args?.join(' ') ?? ''}`);
let runCommand = ref(`${currentProject?.value?.runCommand?.cmd} ${currentProject?.value?.runCommand?.args?.join(' ') ?? ''}`);
let debugCommand = ref(`${currentProject?.value?.debugCommand?.cmd} ${currentProject?.value?.debugCommand?.args?.join(' ') ?? ''}`);


watch(() => currentRoute.params.id as string, (newProjectId, _) => {
  currentProject.value = applicationState?.value?.projects[newProjectId];
  githubBranch.value = currentProject?.value?.configuredGithubBranch;
  githubRepoUrl.value = currentProject?.value?.githubRepoUrl;
  localRepoPath.value = currentProject?.value?.localRepoPath;
  buildCommand.value = `${currentProject?.value?.buildCommand?.cmd} ${currentProject?.value?.buildCommand?.args?.join(' ') ?? ''}`;
  runCommand.value = `${currentProject?.value?.runCommand?.cmd} ${currentProject?.value?.runCommand?.args?.join(' ') ?? ''}`;
  debugCommand.value = `${currentProject?.value?.debugCommand?.cmd} ${currentProject?.value?.debugCommand?.args?.join(' ') ?? ''}`;

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
            <Select id="githubBranch" v-model="githubBranch" :options="currentProject?.githubBranches"></Select>
            <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
              the GitHub
              branch.
            </Message>
          </div>
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="githubRepositoryUrl">Repository url</label>
            <InputText id="githubRepositoryUrl" v-model="githubRepoUrl"/>
            <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
              the GitHub
              repository url.
            </Message>
          </div>
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="localRepoPath">Local repository path</label>
            <InputText id="localRepoPath" v-model="localRepoPath"/>
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
            <InputText id="buildCommand" v-model="buildCommand"/>
            <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
              the build
              command.
            </Message>
          </div>
          <div class="flex flex-col gap-2 mb-4">
            <label class="text-lg" for="runCommand">Run command</label>
            <InputText id="runCommand" v-model="runCommand"/>
            <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
              the run
              command.
            </Message>
          </div>
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="debugCommand">Debug command</label>
            <InputText id="debugCommand" v-model="debugCommand"/>
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
              <router-link v-if="slotProps.data" v-slot="{ href, navigate }"
                           :to="`/project/${slotProps.data}`"
                           custom>
                <a :href="href" @click="navigate">
                  <span>{{ slotProps.data }}</span>
                </a>
              </router-link>
            </template>
          </Column>

        </DataTable>
      </Fieldset>
      <div class="flex flex-row gap-4 m-4 justify-end">
        <Button class="uppercase">Pull code</Button>
        <Button class="uppercase">Build</Button>
        <Button class="uppercase">Run</Button>
        <Button class="uppercase">Debug</Button>
        <Button class="uppercase">Stop</Button>
      </div>
    </div>
  </ScrollPanel>
</template>

