<script lang="ts" setup>
import {useRoute} from "vue-router";
import {inject, ref, watch} from "vue";
import ScrollPanel from "primevue/scrollpanel";
import {Button, DataTable, InputText, Select} from "primevue";
import Fieldset from "primevue/fieldset";
import Column from 'primevue/column';
import {APPLICATION_STATE, RETRIEVE_CODE_CMD, SAVE_PROJECT_CONFIG_CMD} from "../common.ts";
import {ProjectStateOutput} from "../messages/project_state_output.ts";
import {invoke} from "@tauri-apps/api/core";
import {ApplicationStateOutput} from "../messages/application_state_output.ts";
import {ProjectConfigInput} from "../messages/project_config_input.ts";
import {GithubConfigInput} from "../messages/github_config_input.ts";

let currentRoute = useRoute();

let applicationState = inject(APPLICATION_STATE);

let currentProject = ref<ProjectStateOutput | undefined>(applicationState?.value?.projects[currentRoute.params.id as string]);


let projectId = ref(currentRoute.params.id as string);
let githubBranch = ref(currentProject?.value?.configuredGithubBranch);
let githubRepoUrl = ref(currentProject?.value?.githubRepoUrl);
let localRepoPath = ref(currentProject?.value?.localRepoPath);
let buildCommand = ref(`${currentProject?.value?.buildCommand?.cmd} ${currentProject?.value?.buildCommand?.args?.join(' ') ?? ''}`);
let runCommand = ref(`${currentProject?.value?.runCommand?.cmd} ${currentProject?.value?.runCommand?.args?.join(' ') ?? ''}`);
let debugCommand = ref(`${currentProject?.value?.debugCommand?.cmd} ${currentProject?.value?.debugCommand?.args?.join(' ') ?? ''}`);


watch(() => currentRoute.params.id as string, (newProjectId, _) => {
  projectId.value = newProjectId;
  currentProject.value = applicationState?.value?.projects[newProjectId];
  githubBranch.value = currentProject?.value?.configuredGithubBranch;
  githubRepoUrl.value = currentProject?.value?.githubRepoUrl;
  localRepoPath.value = currentProject?.value?.localRepoPath;
  buildCommand.value = `${currentProject?.value?.buildCommand?.cmd} ${currentProject?.value?.buildCommand?.args?.join(' ') ?? ''}`;
  runCommand.value = `${currentProject?.value?.runCommand?.cmd} ${currentProject?.value?.runCommand?.args?.join(' ') ?? ''}`;
  debugCommand.value = `${currentProject?.value?.debugCommand?.cmd} ${currentProject?.value?.debugCommand?.args?.join(' ') ?? ''}`;

});

async function save_project_config() {
  let project_config: ProjectConfigInput = {
    selectedGithubBranch: githubBranch.value ?? "",
    githubRepoUrl: githubRepoUrl.value ?? "",
    buildCommand: buildCommand.value ?? "",
    runCommand: runCommand.value ?? "",
    debugCommand: debugCommand.value ?? "",
    localRepoPath: localRepoPath.value ?? "",
    projectId: projectId.value
  };
  let updatedApplicationState = await invoke<ApplicationStateOutput>(SAVE_PROJECT_CONFIG_CMD, {
    "projectConfigInput": project_config
  });
  console.log(updatedApplicationState)
  if (applicationState) {
    applicationState.value = updatedApplicationState;
  }
}

async function retrieve_code() {
  let github_config_input: GithubConfigInput = {
    username: applicationState?.value?.github?.username ?? "",
    token: applicationState?.value?.github?.token ?? ""
  };
  let project_config_input: ProjectConfigInput = {
    selectedGithubBranch: githubBranch.value ?? "",
    githubRepoUrl: githubRepoUrl.value ?? "",
    buildCommand: buildCommand.value ?? "",
    runCommand: runCommand.value ?? "",
    debugCommand: debugCommand.value ?? "",
    localRepoPath: localRepoPath.value ?? "",
    projectId: projectId.value
  };
  let updatedApplicationState = await invoke<ApplicationStateOutput>(RETRIEVE_CODE_CMD, {
    "githubConfigInput": github_config_input,
    "projectConfigInput": project_config_input
  });
  console.log(updatedApplicationState)
  if (applicationState) {
    applicationState.value = updatedApplicationState;
  }
}
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
        <Button class="uppercase" @click="save_project_config">Save</Button>
        <Button class="uppercase" @click="retrieve_code">Pull code</Button>
        <Button class="uppercase">Build</Button>
        <Button class="uppercase">Run</Button>
        <Button class="uppercase">Debug</Button>
        <Button class="uppercase">Stop</Button>
      </div>
    </div>
  </ScrollPanel>
</template>

