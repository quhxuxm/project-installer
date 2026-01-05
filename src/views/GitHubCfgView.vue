<script lang="ts" setup>

import {inject, ref} from "vue";
import ScrollPanel from "primevue/scrollpanel";
import {Button, InputText} from "primevue";
import Textarea from 'primevue/textarea';
import {APPLICATION_STATE} from "../common.ts";


let applicationState = inject(APPLICATION_STATE);

let githubUsername = ref<string>(applicationState?.value?.github?.username ?? "");
let githubToken = ref<string>(applicationState?.value?.github?.token ?? "");


</script>

<style scoped>

</style>

<template>

  <ScrollPanel class="h-full px-4">
    <div class="flex flex-col gap-4 h-full">
      <h1 class="text-2xl text-primary mb-4">GitHub configuration</h1>

      <div class="flex flex-col gap-2">
        <label class="text-lg" for="username">Username</label>
        <InputText id="username" v-model="githubUsername"/>
        <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter the
          GitHub
          username.
        </Message>
      </div>
      <div class="flex flex-col grow gap-2">
        <label class="text-lg" for="token">Token</label>
        <Textarea id="token" v-model="githubToken" class="grow"/>
        <Message class="text-gray-500 text-sm" severity="secondary" size="small" variant="simple">Enter the
          GitHub
          token.
        </Message>
      </div>

      <div class="flex flex-row gap-4 m-4 justify-end">
        <Button class="uppercase">Save</Button>
      </div>
    </div>
  </ScrollPanel>
</template>

