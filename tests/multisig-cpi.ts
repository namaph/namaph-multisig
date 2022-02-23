import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { MultisigCpi } from '../target/types/multisig_cpi';
import { SerumMultisig } from '../target/types/serum_multisig';
import assert from 'assert';

describe('multisig-cpi', () => {

	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.Provider.env());

	const program = anchor.workspace.MultisigCpi as Program<MultisigCpi>;
	const multisigProgram = anchor.workspace.SerumMultisig as Program<SerumMultisig>;

	it('Is initialized!', async () => {

		// 1. create a multisig	
		// for similicity sake, we will make a multisig with
		// a single owner

		const owner = anchor.web3.Keypair.generate();
		const multisig = anchor.web3.Keypair.generate();
		const [multisigSigner, nonce] = await anchor.web3.PublicKey.findProgramAddress(
			[multisig.publicKey.toBytes()],
			multisigProgram.programId);

		const owners = [owner.publicKey];
		const threshold = new anchor.BN(1);

		await multisigProgram.rpc.createMultisig(owners, threshold, nonce, {
			accounts: {
				multisig: multisig.publicKey
			},
			instructions: [
				await multisigProgram.account.multisig.createInstruction(
					multisig,
					200
				),
			],
			signers: [multisig],
		});

		// 2. create 'Data'
		// a normal Data account

		const data = anchor.web3.Keypair.generate();

		await program.rpc.initialize(multisigSigner, {
			accounts: {
				data: data.publicKey,
				payer: program.provider.wallet.publicKey,
				systemProgram: anchor.web3.SystemProgram.programId
			},
			signers: [data]
		});

		let dataAccount = await program.account.data.fetch(data.publicKey);
		assert.equal(dataAccount.authority.toBase58(), multisigSigner.toBase58());
		assert.equal(dataAccount.value, 0);

		// 3. create an transaction via the multisig
		// 'updateValue'

		const transaction = anchor.web3.Keypair.generate();
		const txSize = 200;
		const instructionData = program.coder.instruction.encode("update_value", {
			value: 42
		});

		const accounts = program.instruction.updateValue.accounts({
			data: data.publicKey,
			authority: multisigSigner
		}) as IAccountMeta[];

		await multisigProgram.rpc.createTransaction(program.programId, accounts, instructionData, {
			accounts: {
				multisig: multisig.publicKey,
				transaction: transaction.publicKey,
				proposer: owner.publicKey
			},
			instructions: [
				await multisigProgram.account.transaction.createInstruction(
					transaction,
					txSize
				)
			],
			signers: [transaction, owner]
		});

		let transactionData = await multisigProgram.account.transaction.fetch(transaction.publicKey);
		assert.equal(transactionData.didExecute, false);

		// (approve) we skip this since it's a single owner multisig,
		// and the proposer approves the transaction when we created it.
		// the transaction is ready to be executed

		const remainingAccounts = accounts.map(meta => meta.pubkey.equals(multisigSigner) ? { ...meta, isSigner: false } : meta).concat({
			pubkey: program.programId,
			isWritable: false,
			isSigner: false,
		});

		// 4. exectue transaction
		await multisigProgram.rpc.executeTransaction({
			accounts: {
				multisig: multisig.publicKey,
				multisigSigner,
				transaction: transaction.publicKey
			},
			remainingAccounts
		})

		dataAccount = await program.account.data.fetch(data.publicKey);
		assert.equal(dataAccount.value, 42);
		nsactionData = await multisigProgram.account.transaction.fetch(transaction.publicKey);
		assert.equal(transactionData.didExecute, true);

	});

});

interface IAccountMeta {
	pubkey: anchor.web3.PublicKey,
	isSigner: boolean,
	isWritable: boolean
}
