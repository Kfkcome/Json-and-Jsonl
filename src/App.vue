<script setup>
import { ref } from "vue";

import hello from "./component/helloworld.vue";
import mainView from "./component/MainView.vue";



</script>

<template>
  <div>
    <!-- <h1>Welcome to your Vue 3 + Vite + Tauri app</h1> -->
    <!-- <hello /> -->
    
    <!-- <mainView /> -->
     <div data-tauri-drag-region class ="header">
      <h1 class="welcome-message">Json View Tool</h1>
     </div>
  </div>
</template>

<style scoped>
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #f5f5f5; /*TODO: 实现动态颜色变化 */
  /* border-bottom: 1px solid #e0e0e0; */
  margin-top: -10px;
  margin-left: -10px;
  margin-right: -10px;
  width: 110%;
  box-sizing: border-box;
}
.welcome-message {
  text-align: left;
  margin-top: 10px;
  margin-left: 100px;
  font-size: 24px;
  color: #333;
}
#custom-window {
  -webkit-app-region: drag;
  /* 使得这部分可以被拖动 */
  background-color: rgba(255, 255, 255, 0.95);
  /* 透明背景 */
  border-radius: 12px;
  /* 设置圆角效果 */
  width: 800px;
  height: 600px;
  /* overflow: hidden; */
}
</style>

