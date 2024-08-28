<script setup lang="ts">
import { ref, onMounted } from "vue";

const show = ref(true);
const emits = defineEmits(["closed"]);
onMounted(() => {
  const t = setTimeout(() => {
    show.value = false;
    emits("closed");
    clearTimeout(t);
  }, 3000);
});
</script>
<template>
  <div
    v-if="show"
    class="fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 p-3 bg-dark/90 text-primary z-10 shadow rounded-md border border-primary min-w-max"
  >
    <slot></slot>
  </div>
</template>
