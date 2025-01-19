<script setup>
import { ref } from "vue";
import MainView from "./component/MainView.vue";

</script>

<template >
    <!-- <mainView /> -->
    
    <div class="app-container">
      
      <div data-tauri-drag-region class="header">
        <h1 data-tauri-drag-region class="welcome-message">Json View Tool</h1>
        <!-- <h2>file name</h2> todo: -->
      </div>

      <MainView />
      <div class="footer">
        <div class="footer-item">Footer content goes here</div>
      </div>
  </div>
</template>

<style scoped>
.app-container {
position: fixed;
display:flex;
flex-direction: column;
height: 100%;
width: 100%;
top: 0;
left: 0;
/* transition: all 0.9s ease; */
}
.footer {
  position: fixed;
  bottom: 0;
  width: 100%;
  background-color: #be4545;
  color: white;
  text-align: center;
  height: 30px;
  /* padding: 4px; */
}
.footer-item {
  position: absolute;
  bottom: 25%;
  left: 20px;
  margin: 0;
  padding: 0;
}
.header {
  position: fixed;
  background-color: #be4545; /*TODO: 实现动态颜色变化 */
  /* border-bottom: 1px solid #e0e0e0; */
  /* margin-top: -10px; */
  height: 40px;
  padding: 5px;
  width: 100%;

}


.welcome-message {
  position: absolute;
  top: 0%;
  left: 80px;
  font-size: 20px;
  -webkit-user-select: none;
  user-select: none;
  -webkit-app-region: no-drag;
  cursor: default;
  /* text-align: left; */
  color: #333333;
}

</style>

