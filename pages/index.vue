<template>
  <div class="container">
    <div class="cool-box">
      <h1
        class="cool-text"
        v-motion
        :initial="{
          opacity: 0,
          y: 100,
          transition: {
            duration: 100,
          },
        }"
        :enter="{
          opacity: 1,
          y: 0,
          scale: 1,
          transition: {
            duration: 800,
          },
        }"
        :variants="{ custom: { scale: 2 } }"
        :delay="300"
      >
        Maria.rs
      </h1>
      <p
        v-motion
        :initial="{
          opacity: 0,
          y: 100,
          transition: {
            duration: 100,
          },
        }"
        :enter="{
          opacity: 1,
          y: 0,
          scale: 1,
          transition: {
            duration: 800,
          },
        }"
        :variants="{ custom: { scale: 2 } }"
        :delay="1200"
      >
        <span> </span> a simplified rust web framework.
      </p>
      <!-- <p>
          Maria is mainly based on
          <a href="https://expressjs.com/">expressjs.</a>you can easly maintain
          your express code to maria code.
        </p> -->
    </div>
    <div class="cursorToDown">
      <div
        class="flex justify-center items-center mt-28"
        v-motion
        :initial="{
          opacity: 0,
          y: -100,
        }"
        :enter="{
          opacity: 1,
          y: 0,
          transition: {
            duration: 1000,
          },
        }"
        :delay="2200"
      >
        <div class="bg-red-400">
          <client-only>
            <Vue3Lottie
              :animationData="scrollLottie"
              :autoPlay="true"
              :height="75"
              :width="75"
            />
          </client-only>
        </div>
      </div>
    </div>
  </div>
  <div>
    <div></div>
    <h1 class="text-3xl">Why?</h1>
    <p>
      Many of Rust frameworks can be too complex and hard to learn for
      developers which came from Nodejs enviroment. Maria highly influenced on
      Expressjs framework, also Maria has an easy syntax.
    </p>
  </div>
</template>
<script setup>
import scrollLottie from "@/assets/scrolldown.json";
</script>
<style>
@import url("https://fonts.googleapis.com/css2?family=Montserrat:ital,wght@0,500;0,600;0,800;1,400;1,500;1,600&display=swap");
.container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

.cool-text {
  font-size: 3rem;
  filter: drop-shadow(3px 3px 0.75rem #fd5c63);
  animation: text-animation 2.5s ease-in-out forwards;
  font-weight: 900;
}
@keyframes text-animation {
  0% {
    filter: drop-shadow(1px 1px 0 #5c7cfd);
  }
  100% {
    filter: drop-shadow(1px 1px 0.75rem #fd5c63);
  }
}
</style>
