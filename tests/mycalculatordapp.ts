const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;
describe("mycalculatordapp", () => {
	const provider = anchor.AnchorProvider.local();
	anchor.setProvider(provider);
  console.log("Available programs in anchor.workspace:", Object.keys(anchor.workspace));

	const calculator = anchor.web3.Keypair.generate();
	const program = anchor.workspace.mycalculatordapp;

	it("Creates a calculator", async () => {
		await program.rpc.create("Welcome to Solana", {
			accounts: {
				calculator: calculator.publicKey,
				user: provider.wallet.publicKey,
				systemProgram: SystemProgram.programId,
			},
			signers: [calculator],
		});
		const account = await program.account.calculator.fetch(
			calculator.publicKey
		);
		assert.ok(account.greeting === "Welcome to Solana");
	});
	it("Adds two numbers", async () => {
		await program.rpc.add(new anchor.BN(2), new anchor.BN(3), {
			accounts: {
				calculator: calculator.publicKey,
			},
		});
		const account = await program.account.calculator.fetch(
			calculator.publicKey
		);
		assert.ok(account.result.eq(new anchor.BN(5)));
	});
	it("Subtracts two numbers", async function () {
		await program.rpc.subtract(new anchor.BN(32), new anchor.BN(33), {
			accounts: {
				calculator: calculator.publicKey,
			},
		});

		const account = await program.account.calculator.fetch(
			calculator.publicKey
		);
		assert.ok(account.result.eq(new anchor.BN(-1)));
	});

	it("Multiplies two numbers", async function () {
		await program.rpc.multiply(new anchor.BN(2), new anchor.BN(3), {
			accounts: {
				calculator: calculator.publicKey,
			},
		});

		const account = await program.account.calculator.fetch(
			calculator.publicKey
		);
		assert.ok(account.result.eq(new anchor.BN(6)));
	});

	it("Divides two numbers", async function () {
		await program.rpc.divide(new anchor.BN(10), new anchor.BN(3), {
			accounts: {
				calculator: calculator.publicKey,
			},
		});

		const account = await program.account.calculator.fetch(
			calculator.publicKey
		);
		assert.ok(account.result.eq(new anchor.BN(3)));
		assert.ok(account.remainder.eq(new anchor.BN(1)));
	});
});