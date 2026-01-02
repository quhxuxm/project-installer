import { createApp } from "vue";
import HomeView from "./view/HomeView.vue";
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';
import "./style.css";
import 'primeicons/primeicons.css'
import {createMemoryHistory, createRouter} from "vue-router";
import GitHubCfgView from "./view/GitHubCfgView.vue";
import ProjectCfgView from "./view/ProjectCfgView.vue";
import JavaCfgView from "./view/JavaCfgView.vue";
import MavenCfgView from "./view/MavenCfgView.vue";
import NodeJsCfgView from "./view/NodeJsCfgView.vue";

const routes = [
    {path: '/java', component: JavaCfgView },
    {path: '/maven', component: MavenCfgView },
    {path: '/nodejs', component: NodeJsCfgView },
    { path: '/github', component: GitHubCfgView },
    {path:'/project/:id', component: ProjectCfgView}
];

const router = createRouter({
    history: createMemoryHistory(),
    routes,
});

let app = createApp(HomeView);
app.use(PrimeVue,{
    theme: {
        preset: Aura
    }
}).use(router);
app.mount("#home");
