<template>
  <div
    class="mt-6 flex justify-center"
    v-motion
    :initial="{
      opacity: 0,
      y: -100,
      transition: {
        duration: 100,
      },
    }"
    :enter="{
      opacity: 1,
      y: 0,
      scale: 1,
      transition: {
        duration: props.duration,
      },
    }"
    :variants="{ custom: { scale: 2 } }"
    :delay="props.delay"
  >
    <slot> </slot>
  </div>
</template>
<script setup>
import { ref, defineProps } from "vue";
const props = defineProps(["delay", "duration"]);
</script>
