<script lang="ts" setup>
import {useRoute} from "vue-router";
import {ref, watch} from "vue";
import ScrollPanel from "primevue/scrollpanel";
import {Button, InputText, Select} from "primevue";
import Fieldset from "primevue/fieldset";
import {GET_PROJECT_CODE_CMD, GET_PROJECT_RUNTIME_DETAIL_CMD,} from "../common.ts";
import {invoke} from "@tauri-apps/api/core";
import {ProjectRuntimeDetail, ProjectRuntimeUpdate} from "../messages/project.ts";

let currentRoute = useRoute();

let loading = ref(true);

let projectRuntimeDetail = ref<ProjectRuntimeDetail>({
  name: "",
  description: "",
  githubBranch: "",
  githubRepoUrl: "",
  localRepoPath: "",
  currentProcess: undefined,
  availableGithubBranches: [],
  buildCommand: undefined,
  runCommand: undefined,
  stopCommand: undefined,
  debugCommand: undefined
});

let buildCommandVal = ref("");
let runCommandVal = ref("");
let debugCommandVal = ref("")

function switchProjectDetail(projectId: string) {
  loading.value = true;
  invoke<ProjectRuntimeDetail>(
      GET_PROJECT_RUNTIME_DETAIL_CMD,
      {projectId}
  ).then((backendData) => {
    projectRuntimeDetail.value = backendData;
    buildCommandVal.value = `${backendData.buildCommand?.command} ${backendData.buildCommand?.args.join(' ')}`.trim();
    runCommandVal.value = `${backendData.runCommand?.command} ${backendData.runCommand?.args.join(' ')}`.trim();
    debugCommandVal.value = `${backendData.debugCommand?.command} ${backendData.debugCommand?.args.join(' ')}`.trim()

    loading.value = false;
  });
}


switchProjectDetail(currentRoute.params.id as string);

watch(
    () => currentRoute.params.id as string,
    (newProjectId, _) => {
      switchProjectDetail(newProjectId);
    },
);


function getProjectCode() {
  let projectRuntimeUpdate: ProjectRuntimeUpdate = {
    buildCommand: buildCommandVal.value,
    debugCommand: debugCommandVal.value,
    githubRepoUrl: projectRuntimeDetail.value.githubRepoUrl,
    localRepoPath: projectRuntimeDetail.value.localRepoPath,
    runCommand: runCommandVal.value,
    projectId: currentRoute.params.id as string,
    githubBranch: projectRuntimeDetail.value.githubBranch
  };
  invoke(GET_PROJECT_CODE_CMD, {
    projectRuntimeUpdate
  }).catch((e) => {
    console.log("Error happen when get project code: " + e);
  })
}

</script>

<style scoped></style>

<template>
  <div v-if="loading" class="h-full flex flex-col justify-center align-center w-full">
    <h1 class="text-2xl text-primary text-center">Loading ...</h1>
  </div>
  <ScrollPanel v-else class="h-full px-4">
    <h1 class="text-2xl text-primary mb-4">
      {{ projectRuntimeDetail.name }}
    </h1>
    <div class="flex flex-col gap-4 ">
      <Fieldset
          class="text-xl  pb-3 pt-3"
          legend="Repository configuration"
          toggleable
      >
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="githubBranch">Branch</label>
            <Select
                id="githubBranch"
                v-model="projectRuntimeDetail.githubBranch"
                :options="projectRuntimeDetail.availableGithubBranches"
            ></Select>
            <Message
                class="text-gray-500 text-sm"
                severity="secondary"
                size="small"
                variant="simple"
            >Enter the GitHub branch.
            </Message>
          </div>
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="githubRepositoryUrl"
            >Repository url</label
            >
            <InputText
                id="githubRepositoryUrl" v-model="projectRuntimeDetail.githubRepoUrl"
                readonly
            />
            <Message
                class="text-gray-500 text-sm"
                severity="secondary"
                size="small"
                variant="simple"
            >Enter the GitHub repository url.
            </Message>
          </div>
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="localRepoPath"
            >Local repository path</label
            >
            <InputText id="localRepoPath" v-model="projectRuntimeDetail.localRepoPath"/>
            <Message
                class="text-gray-500 text-sm flex flex-col"
                severity="secondary"
                size="small"
                variant="simple"
            >
              <span>Enter the local repository path, the concrete local path will be:</span>
              <span class="text-primary">
                {{ projectRuntimeDetail.localRepoPath }}/{{ projectRuntimeDetail.githubBranch }}
              </span>
            </Message>
          </div>
        </div>
      </Fieldset>
      <Fieldset class="text-xl  pb-3 pt-3" legend="Command configuration" toggleable>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-2 mb-4">
            <label class="text-lg" for="buildCommand">Build command</label>
            <InputText id="buildCommand" v-model="buildCommandVal"/>
            <Message
                class="text-gray-500 text-sm"
                severity="secondary"
                size="small"
                variant="simple"
            >Enter the build command.
            </Message>
          </div>
          <div class="flex flex-col gap-2 mb-4">
            <label class="text-lg" for="runCommand">Run command</label>
            <InputText id="runCommand" v-model="runCommandVal"/>
            <Message
                class="text-gray-500 text-sm"
                severity="secondary"
                size="small"
                variant="simple"
            >Enter the run command.
            </Message>
          </div>
          <div class="flex flex-col gap-2">
            <label class="text-lg" for="debugCommand">Debug command</label>
            <InputText id="debugCommand" v-model="debugCommandVal"/>
            <Message
                class="text-gray-500 text-sm"
                severity="secondary"
                size="small"
                variant="simple"
            >Enter the debug command.
            </Message>
          </div>
        </div>
      </Fieldset>
      <div class="flex flex-row gap-4 m-4 justify-end">
        <Button class="uppercase">Save</Button>
        <Button class="uppercase" @click="getProjectCode">Get code</Button>
        <Button class="uppercase">Build</Button>
        <Button class="uppercase">Run</Button>
        <Button class="uppercase">Debug</Button>
        <Button class="uppercase">Stop</Button>
      </div>
    </div>
  </ScrollPanel>
</template>
