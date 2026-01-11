<script lang="ts" setup>
import {useRoute} from "vue-router";
import {ref, watch} from "vue";
import ScrollPanel from "primevue/scrollpanel";
import {Button, InputText, Select} from "primevue";
import Fieldset from "primevue/fieldset";
import {GET_PROJECT_RUNTIME_DETAIL_CMD,} from "../common.ts";
import {invoke} from "@tauri-apps/api/core";
import {ProjectRuntimeDetail} from "../messages/project.ts";

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

function switchProjectDetail(projectId: string) {
  loading.value = true;
  invoke<ProjectRuntimeDetail>(
      GET_PROJECT_RUNTIME_DETAIL_CMD,
      {projectId}
  ).then((backendData) => {
    projectRuntimeDetail.value = backendData;
    loading.value = false;
  });
}

// onMounted(async () => {
//   await switchProjectDetail(currentRoute.params.id as string);
// });

switchProjectDetail(currentRoute.params.id as string);

watch(
    () => currentRoute.params.id as string,
    (newProjectId, _) => {
      switchProjectDetail(newProjectId);
    },
);

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
    <div class="flex flex-col gap-4">
      <Fieldset
          class="text-xl"
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
                id="githubRepositoryUrl"
                v-model="projectRuntimeDetail.githubRepoUrl"
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
                class="text-gray-500 text-sm"
                severity="secondary"
                size="small"
                variant="simple"
            >Enter the local repository path.
            </Message>
          </div>
        </div>
      </Fieldset>
      <Fieldset class="text-xl" legend="Command configuration" toggleable>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-2 mb-4">
            <label class="text-lg" for="buildCommand">Build command</label>
            <InputText id="buildCommand" v-model="projectRuntimeDetail.buildCommand"/>
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
            <InputText id="runCommand" v-model="projectRuntimeDetail.runCommand"/>
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
            <InputText id="debugCommand" v-model="projectRuntimeDetail.debugCommand"/>
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
        <Button class="uppercase">Pull code</Button>
        <Button class="uppercase">Build</Button>
        <Button class="uppercase">Run</Button>
        <Button class="uppercase">Debug</Button>
        <Button class="uppercase">Stop</Button>
      </div>
    </div>
  </ScrollPanel>
</template>
