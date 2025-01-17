<template lang="html">
    <div>
        
    </div>
</template>
<script>
import { onMounted } from 'vue';

export default {
  name: 'HelloWorld',
   setup() {
    onMounted(() => {
      console.log('Mounted!');
    });

    // await someAsyncOperation();  // 放在生命周期钩子之后
  }
};

</script>
<style lang="">
    
</style>