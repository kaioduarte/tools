async function case1() {
  let a = 1;
  const b = (await c());
  if (foo) {
    await f();
  }
  const c = async () => {
    await doSomething()
  }
  d;
}

async function case2() {
	await doSomething()
}

(async function() { await doSomething() })

async () => { await doSomething() }

async () => await doSomething()

({ async foo() { await doSomething() } })

class A { async foo() { await doSomething() } }

(class { async foo() { await doSomething() } })

async function foo() { await (async () => { await doSomething() }) }

// empty functions are ok.
async function foo() {}
async () => {}

// normal functions are ok.
function foo() { doSomething() }
const foo = () => { doSomething() }
const foo = () => doSomething()

// for-await-of
async function foo() { for await (x of xs); }

// global await -- NOT SUPPORTED YET
// await foo()

// for await (let num of asyncIterable) {
// 	console.log(num);
// }

async function* run() { yield * anotherAsyncGenerator() }

async function* run() {
	await new Promise(resolve => setTimeout(resolve, 100));
	yield 'Hello';
	console.log('World');
}

async function* run() { }

const foo = async function *(){}

const foo = async function *(){ console.log("bar") }

async function* run() { console.log("bar") }
