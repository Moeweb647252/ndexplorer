<script setup lang="tsx">
import { onMounted, ref, createVNode, render } from 'vue';
import CommonWindow from './components/CommonWindow.vue';

onMounted(() => {
  for (let i = 0; i < 10; i++) {
    for (let ii = 0; ii < 10; ii++) {
      let el = createVNode(CommonWindow, { x: i * 100, y: ii * 100 });
      render(el, document.body);
    }
  }
});
</script>

<template></template>
