import { createApp } from "vue";
import App from "./App.vue";
import VueDraggableResizable from 'vue-draggable-resizable';

const app =createApp(App)
// .component("vue-draggable-resizable", VueDraggableResizable)

app.mount("#app")
// Vue.component('vue-draggable-resizable', VueDraggableResizable);
// Vue.component("vue-drag-resize", VueDragResize);
