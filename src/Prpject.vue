<script lang="ts" setup>
import {ProjectConfig} from "./config/project_config.ts";

const props = defineProps<{
  project: ProjectConfig
}>();

</script>

<style scoped>

</style>

<template>
  <label :for="'desc-'+ props.project.projectId">
    {{ props.project.description }}
  </label>
  <input :id="'desc-'+ props.project.projectId" :value="props.project.description" type="text"/>
</template>

