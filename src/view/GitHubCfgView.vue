<script setup lang="ts">

import {inject, ref} from "vue";
import {GitHubConfiguration, ToolConfiguration} from "../value/configuration.ts";
import ScrollPanel from "primevue/scrollpanel";
import {InputText} from "primevue";
import {Button} from "primevue";
import Textarea from 'primevue/textarea';


let toolConfiguration=inject<ToolConfiguration>("TOOL_CONF");

let githubConfiguration= ref<GitHubConfiguration | undefined>(toolConfiguration?.github);



</script>

<style scoped>

</style>

<template>

  <ScrollPanel class="h-full px-4">
    <div class="flex flex-col gap-4 h-full">
      <h1 class="text-2xl text-primary mb-4">GitHub configuration</h1>

      <div class="flex flex-col gap-2">
        <label for="username" class="text-lg">Username</label>
        <InputText id="username" />
        <Message class="text-gray-500 text-sm" size="small" severity="secondary" variant="simple">Enter the GitHub username.</Message>
      </div>
      <div class="flex flex-col grow gap-2">
        <label for="token" class="text-lg">Token</label>
        <Textarea id="token" class="grow"  />
        <Message class="text-gray-500 text-sm" size="small" severity="secondary" variant="simple">Enter the GitHub token.</Message>
      </div>

      <div class="flex flex-row gap-4 m-4 justify-end" >
        <Button class="uppercase">Save</Button>
      </div>
    </div>
  </ScrollPanel>
</template>

