<template>
  <div>
    <pre
      class="custom-prism-pre"
    ><code class="language-rust">{{ code }}</code></pre>
  </div>
</template>

<script setup>
import Prism from "prismjs";
import { onMounted, defineProps } from "vue";
import "prismjs/themes/prism-coy.css";
import "prismjs/components/prism-rust";
import "prismjs/components/prism-css";

let props = defineProps(["code"]);

onMounted(() => {
  Prism.highlightAll();
});
</script>

<style>
.custom-prism-pre {
  padding: 0;
  @apply md:p-5 lg:p-10; /* Apply custom padding with Tailwind CSS */
  max-height: 15em; /* Set maximum height to accommodate 10 lines */
  overflow-y: auto; /* Enable vertical scrolling */
}
</style>
