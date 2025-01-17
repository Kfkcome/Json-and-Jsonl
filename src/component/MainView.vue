<template>
    <div >
        <!-- Windows Control button-->
        <div data-tauri-drag-region id="mac-window-controls">
            <button class="close" @click="appWindow.close()" />
            <button class="minimize" @click="appWindow.minimize()" />
            <button class="maximize" @click="appWindow.maximize()" />
        </div>
    </div>
    
</template>

<script setup lang="js">
// You might want to add window control functionality
import { Window } from '@tauri-apps/api/window';

const appWindow = Window.getCurrent();



</script>

<style lang="css" scoped>
#mac-window-controls {
    -webkit-app-region: drag;
    /* Makes the window draggable */
    display: flex;
    align-items: center;
    justify-content: flex-start;
    padding: 10px;
    background: #0000;
    height: 30px;
    border-top-left-radius: 10px;
    border-top-right-radius: 10px;
}

button {
    background: none;
    border: none;
    width: 15px;
    height: 15px;
    border-radius: 50%;
    margin: 0 5px;
    cursor: pointer;
    transition: background-color 0.3s ease;
}

button:hover {
    background-color: rgba(255, 255, 255, 0.2);
}

.close {
    background-color: #ff5f57;
    /* Red */
}

.minimize {
    background-color: #ffbd2e;
    /* Yellow */
}

.maximize {
    background-color: #27c93f;
    /* Green */
}

/* Optional: Customize the hover effect */
button:hover.close {
    background-color: rgba(255, 95, 87, 0.3);
}

button:hover.minimize {
    background-color: rgba(255, 189, 46, 0.3);
}

button:hover.maximize {
    background-color: rgba(39, 201, 63, 0.3);
}

</style>
