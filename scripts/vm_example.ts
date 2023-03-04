const hello = runjs.helloWorld("alice");
console.log("content", hello);

const world = await runjs.secondHelloWorld("alice", "bob");
console.log("content", world);
