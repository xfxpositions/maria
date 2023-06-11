export default defineNuxtConfig({
  devtools: { enabled: true },
  css: ["vue3-lottie/dist/style.css", "@/assets/tailwind.css"],
  modules: ["@vueuse/motion/nuxt", "@nuxtjs/tailwindcss"],

  app: {
    head: {
      title: "Maria.rs",
      charset: "utf-8",
      viewport: "width=device-width, initial-scale=1",
      link: [
        {
          rel: "stylesheet",
          href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/default.min.css",
        },
      ],
      script: [
        {
          src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js",
        },
      ],
    },
  },
});
