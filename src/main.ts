import { createApp } from "vue";
import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";
import "./style.css";
import "primeicons/primeicons.css";
import { createMemoryHistory, createRouter } from "vue-router";
import GitHubCfgView from "./views/GitHubCfgView.vue";
import ProjectCfgView from "./views/ProjectCfgView.vue";
import JavaCfgView from "./views/JavaCfgView.vue";
import MavenCfgView from "./views/MavenCfgView.vue";
import NodeJsCfgView from "./views/NodeJsCfgView.vue";
import App from "./App.vue";


const routes = [
  { path: "/java", component: JavaCfgView },
  { path: "/maven", component: MavenCfgView },
  { path: "/nodejs", component: NodeJsCfgView },
  { path: "/github", component: GitHubCfgView },
  { path: "/project/:id", component: ProjectCfgView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

let app = createApp(App);
app
  .use(PrimeVue, {
    theme: {
      preset: Aura,
    },
  })
  .use(router);
app.mount("#app");
