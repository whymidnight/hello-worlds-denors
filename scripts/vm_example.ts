const hello = runjs.helloWorld("alice");
console.log(hello);

const world = await runjs.secondHelloWorld("alice", "bob");
console.log(world);
