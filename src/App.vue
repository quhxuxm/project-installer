<script lang="ts" setup>
import {ref} from "vue";
import {ToolConfig} from "./config/tool_config.ts";
import {ProjectConfig} from "./config/project_config.ts";
import Prpject from "./Prpject.vue";

const toolConfig = ref<ToolConfig>(initToolConfig());
const enteredProjectId = ref<string>("");


/**
 * Initialize the project configuration
 */
function initToolConfig(): ToolConfig {
  const toolConfig: ToolConfig = new ToolConfig("quhxuxm", "githubtoken");
  const rgsProject = new ProjectConfig("rgs",
      "https://github.com/igt-playcasino/rgs.git", "6.9.0-develop",
      "D:\\Workspace\\Git\\rgs");
  rgsProject.description = "RGS";
  const competitionProject = new ProjectConfig("competition",
      "https://github.com/igt-playcasino/competition.git", "6.9.0-develop",
      "D:\\Workspace\\Git\\competition");
  competitionProject.description = "Competition";
  const pasProject = new ProjectConfig("pas",
      "https://github.com/igt-playcasino/pas.git", "6.9.0-develop",
      "D:\\Workspace\\Git\\pas");
  pasProject.description = "PAS";
  const cfgsProject = new ProjectConfig("cfgs",
      "https://github.com/igt-playcasino/cfgs.git", "6.9.0-develop",
      "D:\\Workspace\\Git\\cfgs");
  cfgsProject.description = "CFGS";
  toolConfig.addProject(rgsProject);
  toolConfig.addProject(competitionProject);
  toolConfig.addProject(pasProject);
  toolConfig.addProject(cfgsProject);
  return toolConfig;
}
</script>

<template>
  <main class="container">
    <h1>Project management tool</h1>
    <ul>
      <li v-for="project in toolConfig.getAllProjects()">
        <Prpject :project="project"/>
      </li>
    </ul>
  </main>
</template>

<style scoped>
</style>

