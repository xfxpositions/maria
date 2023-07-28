<template>
  <div class="container">
    <div class="mb-12 box-1">
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
        <div
          class="mt-6 flex justify-center"
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
          :delay="1700"
        >
          <a
            style="letter-spacing: -1px"
            class="bg-black font-bold rounded-full py-1 px-2 underline get-started"
            href="https://docs.mariars.dev"
            >Get started.</a
          >
        </div>

        <!-- <p>
          Maria is mainly based on
          <a href="https://expressjs.com/">expressjs.</a>you can easly maintain
          your express code to maria code.
        </p> -->
      </div>

      <div class="cursorToDown">
        <div
          class="flex justify-center items-center mt-5"
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
          <div class="">
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
  </div>
</template>
<script setup>
import scrollLottie from "@/assets/scrolldown.json";
</script>
<style>
.perde {
  backdrop-filter: blur(10px);
}
.container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  top: 0;
  left: 0;
  width: 100%;
  height: 100vh;
  transform-style: preserve-3d;
}
.box-1 {
  /* transform: translateZ(-10px) scale(2); */
}
.cool-box {
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
.card {
  background: rgba(92, 144, 163, 0.28);
  border-radius: 16px;
  box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(8.6px);
  -webkit-backdrop-filter: blur(8.6px);
  border: 1px solid rgba(92, 144, 163, 0.3);
  box-shadow: inset 26px 0px 18px 6px rgba(0, 0, 0, 0.1);
  filter: drop-shadow(1px 1px 0.75rem rgb(105, 105, 105));
  transition: all 300ms ease-in-out;
}
.get-started {
  background: rgba(255, 255, 255, 0.3);
  border-radius: 16px;
  box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(7.7px);
  -webkit-backdrop-filter: blur(7.7px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  box-shadow: inset 26px 0px 18px 6px rgba(0, 0, 0, 0.1);
  filter: drop-shadow(1px 1px 0.75rem rgb(105, 105, 105));
  transition: all 300ms ease-in-out;
}
.get-started:hover {
  filter: drop-shadow(1px 1px 0.9rem rgb(21, 21, 21));
}
</style>
