<script lang="ts" setup>
import { h, reactive, ref } from 'vue';

const props = defineProps<{
  x: number;
  y: number;
}>();

const HOVER_RANGE = 10;

const pos = reactive({
  x: props.x,
  y: props.y,
});

const size = reactive({
  width: 100,
  height: 100,
});

const cursor = ref('auto');

const move = (e: MouseEvent) => {
  if (e.buttons && e.button == 0) {
    pos.x += e.movementX;
    pos.y += e.movementY;
  }
};

const hover = (e: MouseEvent) => {
  if (e.target === e.currentTarget) {
    if (
      Math.abs(e.clientX - pos.x) <= HOVER_RANGE &&
      Math.abs(e.clientY - pos.y) <= HOVER_RANGE
    ) {
      cursor.value = 'nw-resize';
      return;
    }
    if (
      Math.abs(e.clientX - (pos.x + size.width)) <= HOVER_RANGE &&
      Math.abs(e.clientY - pos.y) <= HOVER_RANGE
    ) {
      cursor.value = 'ne-resize';
      return;
    }
    if (
      Math.abs(e.clientX - pos.x) <= HOVER_RANGE &&
      Math.abs(e.clientY - (pos.y + size.height)) <= HOVER_RANGE
    ) {
      cursor.value = 'sw-resize';
      return;
    }
    if (
      Math.abs(e.clientX - (pos.x + size.width)) <= HOVER_RANGE &&
      Math.abs(e.clientY - (pos.y + size.height)) <= HOVER_RANGE
    ) {
      cursor.value = 'se-resize';
      return;
    }
    if (Math.abs(e.clientX - pos.x) <= HOVER_RANGE) {
      cursor.value = 'w-resize';
      return;
    }
    if (Math.abs(e.clientX - (pos.x + size.width)) <= HOVER_RANGE) {
      cursor.value = 'e-resize';
      return;
    }
    if (Math.abs(e.clientY - pos.y) <= HOVER_RANGE) {
      cursor.value = 'n-resize';
      return;
    }
    if (Math.abs(e.clientY - (pos.y + size.height)) <= HOVER_RANGE) {
      cursor.value = 's-resize';
      return;
    }
  }
};

const resize = (e: MouseEvent) => {
  if (e.buttons && e.button == 0 && e.target === e.currentTarget) {
    if (
      Math.abs(e.clientX - pos.x) <= HOVER_RANGE &&
      Math.abs(e.clientY - pos.y) <= HOVER_RANGE
    ) {
      size.width -= e.movementX;
      size.height - e.movementY;
      pos.x += e.movementX;
      pos.y += e.movementY;
      return;
    }
    if (
      Math.abs(e.clientX - (pos.x + size.width)) <= HOVER_RANGE &&
      Math.abs(e.clientY - pos.y) <= HOVER_RANGE
    ) {
      size.width += e.movementX;
      size.height += e.movementY;
      return;
    }
    if (
      Math.abs(e.clientX - pos.x) <= HOVER_RANGE &&
      Math.abs(e.clientY - (pos.y + size.height)) <= HOVER_RANGE
    ) {
      size.width -= e.movementX;
      size.height += e.movementY;
      pos.x += e.movementX;
      return;
    }
    if (
      Math.abs(e.clientX - (pos.x + size.width)) <= HOVER_RANGE &&
      Math.abs(e.clientY - (pos.y + size.height)) <= HOVER_RANGE
    ) {
      size.width += e.movementX;
      size.height += e.movementY;
      return;
    }
    if (Math.abs(e.clientX - pos.x) <= HOVER_RANGE) {
      size.width -= e.movementX;
      pos.x += e.movementX;
      return;
    }
    if (Math.abs(e.clientX - (pos.x + size.width)) <= HOVER_RANGE) {
      size.width += e.movementX;
      return;
    }
    if (Math.abs(e.clientY - pos.y) <= HOVER_RANGE) {
      size.height -= e.movementY;
      pos.y += e.movementY;
      return;
    }
    if (Math.abs(e.clientY - (pos.y + size.height)) <= HOVER_RANGE) {
      size.height += e.movementY;
      return;
    }
  }
};

const out = (e: MouseEvent) => {
  cursor.value = 'auto';
};
</script>

<template>
  <div
    style="
      position: absolute;
      user-select: none;
      background-color: white;
      color: black;
      border: 1px solid gray;
      border-radius: 5px;
      overflow: hidden;
      padding: 0.3em;
      box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    "
    :style="{
      left: pos.x + 'px',
      top: pos.y + 'px',
      width: size.width + 'px',
      height: size.height + 'px',
      cursor: cursor,
    }"
    v-on:mouseover="hover"
    v-on:mouseout="out"
    v-on:mousemove="resize"
  >
    <div
      style="
        display: grid;
        grid-template-rows: auto 1fr;
        width: 100%;
        height: 100%;
      "
    >
      <div style="height: 2em" @mousemove="move">
        <div style="display: flex; justify-content: space-between">
          <div>Title</div>
          <div>_ 口 X</div>
        </div>
      </div>
      <div style="width: 100%; height: 100%; overflow: hidden">
        <slot></slot>
      </div>
    </div>
  </div>
</template>
