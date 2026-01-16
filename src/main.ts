import {createApp} from "vue";
import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";
import "./style.css";
import "primeicons/primeicons.css";
import {createMemoryHistory, createRouter} from "vue-router";
import GitHubDetailCfgView from "./views/GitHubDetailCfgView.vue";
import App from "./App.vue";
import ProjectDetailCfgView from "./views/ProjectDetailCfgView.vue";
import {ToastService} from "primevue";


const routes = [
    {path: "/github", component: GitHubDetailCfgView},
    {path: "/project/:id", component: ProjectDetailCfgView},
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
    }).use(ToastService)
    .use(router);
app.mount("#app");
