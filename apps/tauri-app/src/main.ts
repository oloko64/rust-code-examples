import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: () => import('./Home.vue') },
        { path: '/second-page', component: () => import('./components/SecondWindow.vue') },
    ]
})

createApp(App).use(router).mount("#app");
