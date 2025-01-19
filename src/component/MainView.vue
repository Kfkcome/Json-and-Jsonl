<template>
    <div class="colomnContainer">
        <div class="colomnItem" :style="{ width: `${initWidth[0]}px` }">
            <div class="rowLayout">row 1</div>
            <div class="resize-handle" @mousedown="(e) => initDrag(e, 0)"></div>
        </div>
        <div class="colomnItem" :style="{ width: `${initWidth[1]}px` }">
            <div class="rowLayout">row 2</div>
            <div class="resize-handle" @mousedown="(e) => initDrag(e, 1)"></div>
        </div>
        <div class="colomnItem" :style="{ width: `${initWidth[2]}px` }">
            <div class="rowLayout">row 3</div>
        </div>
    </div>
</template>


<script setup lang="ts">
import { event } from '@tauri-apps/api'
import { ref } from 'vue'

const isDrag = ref(true)
const startX = ref()
const startWidth = ref()

const initWidth = ref([window.innerWidth / 3, window.innerWidth / 3, window.innerWidth / 3])

const isDragging = ref(false)

const minWidth = window.innerWidth / 6
const maxWidth = window.innerWidth - minWidth

const index = ref(0)

/** 定义一个函数，用于将数值限制在指定的最小值和最大值之间 */
const clamp = (value: number, min: number, max: number) => {
    return Math.max(min,value)
    // return Math.min(Math.max(value, min), max) // 使用 Math.min 和 Math.max 函数来限制数值范围
}

const initDrag = (e: MouseEvent, num: number) => {
    if (!isDrag.value) return
    startX.value = e.clientX
    index.value = num
    startWidth.value = initWidth.value[index.value]
    isDragging.value = true

    console.log('initDrag index:', index.value)
    document.addEventListener('mousemove', doDrag, false)
    document.addEventListener('mouseup', stopDrag, false)
}

/** 定义一个函数，在鼠标拖动时调用 */
const doDrag = (e: MouseEvent) => {
    // console.log('doDrag index:', index)
    // 使用 requestAnimationFrame 来处理动画，确保动画在下一帧渲染前执行
    requestAnimationFrame(() => {
        // 计算新的宽度
        const newWidth = startWidth.value + e.clientX - startX.value
        // 如果新宽度不等于最大宽度，则更新宽度值
        // if (newWidth !== maxWidth) {
            initWidth.value[index.value] = clamp(newWidth, minWidth, maxWidth) // 使用 clamp 函数限制宽度值在最小值和最大值之间
        // }
        tuneWidth()
    })
}

const stopDrag = () => {
    console.log('stopDrag index:', index.value)
    // const boundDoDrag = (event: MouseEvent) => doDrag(event, index)
    document.removeEventListener('mousemove', doDrag, false)
    // document.removeEventListener('mouseup', stopDrag, false)
    document.removeEventListener('mouseup', stopDrag, false)
    isDragging.value = false
}

const tuneWidth = () => {
    const totalWidth = initWidth.value.reduce((acc, w) => acc + w, 0)
    const remainingWidth = window.innerWidth - totalWidth
    if (remainingWidth > 0) {
        initWidth.value[2] += remainingWidth
    }
}

window.addEventListener('resize', () => {
    const totalWidth = window.innerWidth
    initWidth.value = [totalWidth / 3, totalWidth / 3, totalWidth / 3]
    tuneWidth()
})

</script>

<style lang="css" scoped>

.colomnContainer {
    /* flex:1; */
    position: fixed;
    bottom: 30px;
    display: flex;
    flex-direction: row;
    width: 100%;
    height: calc(100% - 80px); /* Adjust the height to not cover the bottom bar */
}
.resize-handle {
    /* flex: 2; */
    width: 3px;
    cursor: ew-resize;
    z-index: 9999;
    background: transparent;
}

.colomnItem {
    /* resize: horizontal; */
    flex-basis: 1;
    display: flex;
    flex-direction: row;
    height: 100%;
    /* overflow: hidden; */
}

.rowLayout {
    flex-grow: 1;
    flex-shrink: 1;
    flex-basis: 0;
    border: 1px solid black;
    /* resize: horizontal; */
    min-width: 100px;
    overflow: auto;
}

</style>
