export default defineNuxtConfig({
  devtools: { enabled: true },
  css: ["vue3-lottie/dist/style.css", "@/assets/tailwind.css"],
  modules: ["@vueuse/motion/nuxt"],
  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },
  buildModules: [
    // ...
    "@nuxtjs/tailwindcss",
  ],
});
