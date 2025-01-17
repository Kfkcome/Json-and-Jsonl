<script setup>
import { ref } from "vue";


</script>

<template >
    <!-- <mainView /> -->
    <div class="app-container">
      
      <div data-tauri-drag-region class="header">
        <h1 data-tauri-drag-region class="welcome-message">Json View Tool</h1>
        <!-- <h2>file name</h2> todo: -->
      </div>
  </div>
</template>

<style scoped>
.app-container {
position: fixed;
height: 100%;
width: 100%;
top: 0;
left: 0;
/* transition: all 0.9s ease; */
}
.header {
  display: flex;
  background-color: #be4545; /*TODO: 实现动态颜色变化 */
  /* border-bottom: 1px solid #e0e0e0; */
  margin-top: -10px;
  padding: 5px;
  width: 100%;

}


.welcome-message {
  -webkit-user-select: none;
  user-select: none;
  -webkit-app-region: no-drag;
  cursor: default;
  text-align: left;
  margin-top: 15px;
  margin-left: 80px;
  font-size: 20px;
  color: #333333;
}

</style>

