<template>
  <div style="height: 500px; width: 500px; border: 1px solid red; position: relative;">
    <vue-draggable-resizable :w="100" :h="100" :parent="true">
      <p>Hello! I'm a flexible component. You can drag me around and you can resize me.</p>
    </vue-draggable-resizable>
  </div>
</template>

<style>
.resizable[data-v-68b9db08] {
  height: 100%;
  position: relative;
  overflow: hidden;
  background: var(--center-bg-color);
}


</style>

<script>


export default {
  data: function () {
    return {
      width: 0,
      height: 0,
      x: 0,
      y: 0
    }
  },
  methods: {
    onResize: function (x, y, width, height) {
      this.x = x
      this.y = y
      this.width = width
      this.height = height
    },
    onDrag: function (x, y) {
      this.x = x
      this.y = y
    }
  }
}
</script>