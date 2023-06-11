<template>
  <div
    class="w-full h-[50px] flex flex-row sticky justify-center mr-[300px] gap-5"
  >
    <Section :duration="800" :delay="2500">
      <a
        href=""
        class="bg-opacity-75 bg-slate-50 hover:bg-slate-100 transition-colors ease-in-out duration-150 p-2 flex justify-center items-center rounded-md"
        >Docs</a
      >
    </Section>
    <Section :duration="800" :delay="2550">
      <a
        href=""
        class="bg-opacity-75 bg-slate-50 hover:bg-slate-100 transition-colors ease-in-out duration-150 p-2 flex justify-center items-center rounded-md"
        >Source</a
      >
    </Section>
    <Section :duration="800" :delay="2600">
      <a
        href=""
        class="bg-opacity-75 bg-slate-50 hover:bg-slate-100 transition-colors ease-in-out duration-150 p-2 flex justify-center items-center rounded-md"
        >Examples</a
      >
    </Section>
  </div>
</template>
