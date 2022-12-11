async function foo() {
	doSomething();
}

(async function () {
	doSomething();
});

async () => {
	doSomething();
};

async () =>
	doSomething()({
		async foo() {
			doSomething();
		},
	});

class A {
	async foo() {
		doSomething();
	}
}

(class {
	async foo() {
		doSomething();
	}
})(
	class {
		async ""() {
			doSomething();
		}
	},
);

async function foo() {
	async () => {
		await doSomething();
	};
}

async function foo() {
	await (async () => {
		doSomething();
	});
}

const foo = async () => {
	console.log();
}
