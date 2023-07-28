<template>
  <div
    class="box-border py-[150px] flex flex-col md:flex-row p-12 gap-16 md:gap-12"
  >
    <div class="card px-6 py-3 md:p-10 lg:p-16 flex-1">
      <h1 class="text-3xl font-bold relative bottom-[10px]">Simple Usage</h1>
      <p>
        Many of Rust frameworks can be too complex and hard to learn for
        developers which came from Nodejs enviroment. Maria highly influenced on
        <a href="" class="underline font-bold">Expressjs</a> framework, also
        Maria has an easy syntaxxxx.
      </p>
      <code
        >warning: this page still in heavy construction. Document can be found
        in <a href="https://docs.rs/maria">docs.rs</a>
      </code>
    </div>
    <div class="flex-1">
      <div class="flex flex-row gap-3">
        <div class="flex-1 text-center">Hello world</div>
        <div class="flex-1 text-center">Json</div>
        <div class="flex-1 text-center">Header</div>
        <div class="flex-1 text-center">Middleware</div>
      </div>
      <Highligh :code="getExampleCode(currentExampleCode).code"></Highligh>
    </div>
  </div>
</template>
<script setup>
import { ref } from "vue";
const currentExampleCode = ref("helloWorld");
const exampleCodes = [
  {
    name: "helloWorld",
    code: `use maria::{Router, Response, Request, HandlerFn, Mutex, Arc};

#[tokio::main]
async fn main(){

    //define first handler
    let home: HandlerFn = Arc::new(move |req: Arc<Mutex<Request>>, res: Arc<Mutex<Response>>|{
        Box::new(async move{
            let mut res = res.lock().await;
            res.send_html("Hello from maria.rs!");
        })
    });

    //create a new router for our app
    let mut router = Router::new();

    router.get("/", vec![home]);


    //that's it!
    router.listen(8080).await;
}
`,
  },
];
const getExampleCode = (codeName) => {
  return exampleCodes.find((code) => code.name == codeName);
};
</script>
