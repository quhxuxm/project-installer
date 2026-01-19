<script lang="ts" setup>
import {useRoute} from "vue-router";
import {ref, watch} from "vue";
import {Button, Column, DataTable, DataTableCellEditCompleteEvent, InputText, Select, SplitButton, Tag} from "primevue";
import Fieldset from "primevue/fieldset";
import {
    EXEC_BUILD_PROCESS,
    EXEC_RUN_PROCESS,
    GET_PROJECT_CODE_CMD,
    GET_PROJECT_RUNTIME_DETAIL_CMD,
    SAVE_PROJECT_CMD
} from "../common.ts";
import {Channel, invoke} from "@tauri-apps/api/core";
import {RunningCommandStatus,ProjectRuntimeDetail, ProjectRuntimeUpdate} from "../messages/project.ts";

let currentRoute = useRoute();

let loading = ref(true);

let projectRuntimeDetailRef = ref<ProjectRuntimeDetail>({
    customizedBuildCommand: undefined,
    customizedDebugCommand: undefined,
    customizedRunCommand: undefined,
    customizedStopCommand: undefined,
    name: "",
    description: "",
    workingBranch: "",
    remoteRepoUrl: "",
    localRepoPath: "",
    currentRunningCommandStatus: undefined,
    availableBranches: [],
    buildCommand: undefined,
    runCommand: undefined,
    stopCommand: undefined,
    debugCommand: undefined,
    customizedProperties: [],
});

let buildCommandRef = ref();
let runCommandRef = ref();
let debugCommandRef = ref();
let currentRunningCommandStatusRef = ref<RunningCommandStatus | undefined | null>();

function switchProjectDetail(projectId: string) {
    loading.value = true;
    invoke<ProjectRuntimeDetail>(GET_PROJECT_RUNTIME_DETAIL_CMD, {projectId}).then((backendData) => {
        projectRuntimeDetailRef.value = backendData;
        buildCommandRef.value = backendData.customizedBuildCommand ?? backendData.buildCommand;
        runCommandRef.value = backendData.customizedRunCommand ?? backendData.runCommand;
        debugCommandRef.value = backendData.customizedDebugCommand ?? backendData.debugCommand;
        loading.value = false;
        actionButtonDisable.value = !!backendData.currentRunningCommandStatus;
        currentRunningCommandStatusRef.value = backendData.currentRunningCommandStatus;
        for (let prop of projectRuntimeDetailRef.value.customizedProperties) {
            console.log(`Customized property - key: ${prop.key}, value: ${prop.value}`);
        }
    });
}

switchProjectDetail(currentRoute.params.id as string);

watch(
    () => currentRoute.params.id as string,
    (newProjectId, _) => {
        switchProjectDetail(newProjectId);
    },
);

function generateProjectUpdate(): ProjectRuntimeUpdate {
    return {
        buildCommand: buildCommandRef.value,
        debugCommand: debugCommandRef.value,
        remoteRepoUrl: projectRuntimeDetailRef.value.remoteRepoUrl,
        localRepoPath: projectRuntimeDetailRef.value.localRepoPath,
        runCommand: runCommandRef.value,
        projectId: currentRoute.params.id as string,
        workingBranch: projectRuntimeDetailRef.value.workingBranch,
        customizedProperties: projectRuntimeDetailRef.value.customizedProperties,
    };
}

let actionButtonDisable = ref(false);

function getProjectCode() {
    let commandStatusChannel = new Channel<RunningCommandStatus>();
    commandStatusChannel.onmessage = (runningCommandStatus) => {
        actionButtonDisable.value = false;
        currentRunningCommandStatusRef.value = runningCommandStatus;
    };
    actionButtonDisable.value = true;
    let projectRuntimeUpdate = generateProjectUpdate();
    invoke(GET_PROJECT_CODE_CMD, {
        projectRuntimeUpdate,
        commandStatusChannel,
    }).catch((e) => {
        actionButtonDisable.value = false;
        console.log("Error happen when get project code: " + e);
    });
}

function saveProject() {
    let commandStatusChannel = new Channel<RunningCommandStatus>();
    commandStatusChannel.onmessage = (runningCommandStatus) => {
        actionButtonDisable.value = false;
        currentRunningCommandStatusRef.value = runningCommandStatus;
    };
    actionButtonDisable.value = true;
    let projectRuntimeUpdate = generateProjectUpdate();
    invoke(SAVE_PROJECT_CMD, {
        projectRuntimeUpdate,
        commandStatusChannel
    })
        .then(() => {
            actionButtonDisable.value = false;
        })
        .catch((e) => {
            actionButtonDisable.value = false;
            console.log("Error happen when get project code: " + e);
        });
}

function execBuildProcess() {
    let commandStatusChannel = new Channel<RunningCommandStatus>();
    commandStatusChannel.onmessage = (runningCommandStatus) => {
        actionButtonDisable.value = false;
        console.log(runningCommandStatus)
        if (runningCommandStatus.status === "running") {
            currentRunningCommandStatusRef.value = runningCommandStatus;
        } else {
            console.log("Remove currentRunningCommandStatusVal")
            currentRunningCommandStatusRef.value = undefined;
        }
    };
    actionButtonDisable.value = true;
    let projectRuntimeUpdate = generateProjectUpdate();
    invoke(EXEC_BUILD_PROCESS, {
        projectRuntimeUpdate,
        commandStatusChannel,
    }).catch((e) => {
        actionButtonDisable.value = false;
        console.log("Error happen when get project code: " + e);
    });
}

function execRunProcess() {
    let commandStatusChannel = new Channel<RunningCommandStatus>();
    commandStatusChannel.onmessage = (runningCommandStatus) => {
        actionButtonDisable.value = false;
        if (runningCommandStatus.status == "running") {
            currentRunningCommandStatusRef.value = runningCommandStatus;
        } else {
            currentRunningCommandStatusRef.value = undefined;
        }
    };
    actionButtonDisable.value = true;
    let projectRuntimeUpdate = generateProjectUpdate();
    invoke(EXEC_RUN_PROCESS, {
        projectRuntimeUpdate,
        commandStatusChannel,
    }).catch((e) => {
        actionButtonDisable.value = false;
        console.log("Error happen when get project code: " + e);
    });
}

function onCPEditComplete(event: DataTableCellEditCompleteEvent) {
    let {data, newValue, field} = event;
    data[field] = newValue;
}

function deleteCPProperty(key: string) {
    projectRuntimeDetailRef.value.customizedProperties = projectRuntimeDetailRef.value.customizedProperties.filter((prop) => prop.key !== key);
}

function addCPProperty() {
    let emptyItem = projectRuntimeDetailRef.value.customizedProperties.find((prop) => prop.key.trim().length == 0);
    if (emptyItem) {
        return;
    }
    projectRuntimeDetailRef.value.customizedProperties.push({
        key: "",
        value: "",
    });
}

const actionCommands = [
    {
        label: "CLONE",
        icon: "pi pi-github",
        command: getProjectCode,
    },
    {
        label: "BUILD",
        icon: "pi pi-hammer",
        command: execBuildProcess,
    },
    {
        label: "RUN",
        icon: "pi pi-caret-right",
        command: execRunProcess,
    },
    {
        label: "DEBUG",
        icon: "pi pi-lightbulb",
        command: execBuildProcess,
    },
];
</script>

<style scoped></style>

<template>
    <div v-if="loading" class="h-full flex flex-col justify-center align-center w-full">
        <h1 class="text-2xl text-primary text-center">Loading ...</h1>
    </div>
    <div v-else class="h-full w-full flex flex-col gap-4 justify-self-center">
        <div class="text-2xl text-primary mb-4 uppercase flex flex-row gap-4 items-center">
            {{ projectRuntimeDetailRef.name +currentRunningCommandStatusRef?.commandType}}
            <Tag v-if="currentRunningCommandStatusRef?.commandType==='build'" value="Building"></Tag>
            <Tag v-if="currentRunningCommandStatusRef?.commandType==='run'" value="Running"></Tag>
            <Tag v-if="currentRunningCommandStatusRef?.commandType==='debug'" value="Debugging"></Tag>
        </div>
        <div class="flex flex-col gap-4">
            <Fieldset class="text-xl pb-3 pt-3" legend="REPOSITORY CONFIGURATION" toggleable>
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-2">
                        <label class="text-lg" for="githubBranch">Branch</label>
                        <Select id="githubBranch" v-model="projectRuntimeDetailRef.workingBranch"
                                :options="projectRuntimeDetailRef.availableBranches"></Select>
                        <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
                            the GitHub branch.
                        </Message>
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="text-lg" for="githubRepositoryUrl">Repository url</label>
                        <InputText id="githubRepositoryUrl" v-model="projectRuntimeDetailRef.remoteRepoUrl" readonly/>
                        <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
                            the GitHub repository url.
                        </Message>
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="text-lg" for="localRepoPath">Local repository path</label>
                        <InputText id="localRepoPath" v-model="projectRuntimeDetailRef.localRepoPath"/>
                        <Message class="text-gray-500 text-sm flex flex-col gap-2" severity="secondary" size="small"
                                 variant="simple">
                            <span>Enter the local repository path.</span>
                            <span>The concrete local path will be:</span>
                            <span class="text-primary"> {{
                                    projectRuntimeDetailRef.localRepoPath
                                }}\{{ projectRuntimeDetailRef.workingBranch }} </span>
                            <span>The customized properties directory path will be:</span>
                            <span class="text-primary"> {{
                                    projectRuntimeDetailRef.localRepoPath
                                }}\{{ projectRuntimeDetailRef.workingBranch }}-configuration </span>
                        </Message>
                    </div>
                </div>
            </Fieldset>
            <Fieldset class="text-xl pb-3 pt-3" legend="COMMAND CONFIGURATION" toggleable>
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-2 mb-4">
                        <label class="text-lg" for="buildCommand">Build command</label>
                        <InputText id="buildCommand" v-model="buildCommandRef"/>
                        <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
                            the build command.
                        </Message>
                    </div>
                    <div class="flex flex-col gap-2 mb-4">
                        <label class="text-lg" for="runCommand">Run command</label>
                        <InputText id="runCommand" v-model="runCommandRef"/>
                        <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
                            the run command.
                        </Message>
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="text-lg" for="debugCommand">Debug command</label>
                        <InputText id="debugCommand" v-model="debugCommandRef"/>
                        <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter
                            the debug command.
                        </Message>
                    </div>
                </div>
            </Fieldset>

            <Fieldset class="text-xl pb-3 pt-3" legend="CISTOMIZED PROPERTIES" toggleable>
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-2 mb-4">
                        <DataTable
                            :value="projectRuntimeDetailRef.customizedProperties"
                            class="w-full"
                            edit-mode="cell"
                            scroll-height="400px"
                            scrollable
                            @cell-edit-complete="onCPEditComplete"
                        >
                            <Column body-class="text-xs w-5/11" field="key" header="Key"
                                    header-class="text-sm text-primary">
                                <template #editor="{ data, field }">
                                    <InputText v-model="data[field]" autofocus class="text-xs! w-fit h-fit" fluid
                                               style="padding: 0; margin: 0; border: 0"/>
                                </template>
                            </Column>
                            <Column body-class="text-xs w-5/11" field="value" header="Value"
                                    header-class="text-sm text-primary">
                                <template #editor="{ data, field }">
                                    <InputText v-model="data[field]" autofocus class="text-xs! w-fit h-fit" fluid
                                               style="padding: 0; margin: 0; border: 0"/>
                                </template>
                            </Column>
                            <Column body-class="text-xs w-1/11" field="value" header-class="text-sm text-primary">
                                <template #body="{ data }">
                                    <Button icon="pi pi-times" rounded severity="danger" size="small" variant="text"
                                            @click="deleteCPProperty(data['key'])"/>
                                </template>
                            </Column>
                        </DataTable>
                        <div class="flex flex-row justify-center">
                            <Button icon="pi pi-plus" rounded severity="primary" size="small"
                                    @click="addCPProperty"></Button>
                        </div>
                    </div>
                </div>
            </Fieldset>
            <div class="flex flex-row gap-4 m-4 mb-8 justify-end">
                <SplitButton
                    :disabled="actionButtonDisable"
                    :model="actionCommands"
                    class="uppercase"
                    dropdownIcon="pi pi-cog"
                    icon="pi pi-check"
                    label="SAVE"
                    @click="saveProject"
                ></SplitButton>
            </div>
        </div>
    </div>
</template>
