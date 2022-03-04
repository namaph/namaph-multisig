import * as anchor from '@project-serum/anchor';
import { BorshInstructionCoder, Program } from '@project-serum/anchor';
import { NamaphMultisig as Namaph } from '../target/types/namaph_multisig';
import { SerumMultisig } from '../target/types/serum_multisig';
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import assert from 'assert';

interface ITransactionAccount {
	pubkey: PublicKey,
	isWritable: boolean,
	isSigner: boolean
};

describe('multisig-cpi', () => {

	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.Provider.env());

	const program = anchor.workspace.NamaphMultisig as Program<Namaph>;
	const multisigProgram = anchor.workspace.SerumMultisig as Program<SerumMultisig>;

	const systemProgram = anchor.web3.SystemProgram.programId;

	it('Is initialized!', async () => {

		// init -------------------------------------------------------

		const username = 'test';
		const mapName = 'namaph';

		const multisig = Keypair.generate();

		const [signer, nonce] = await PublicKey.findProgramAddress([multisig.publicKey.toBytes()],
			multisigProgram.programId);

		const [topology] = await PublicKey.findProgramAddress([Buffer.from('topology'), Buffer.from(mapName.slice(0, 32))], program.programId);

		const [membership] = await PublicKey.findProgramAddress([Buffer.from('membership'), program.provider.wallet.publicKey.toBytes()],
			program.programId);

		await program.rpc.initialize(username, mapName, 10, nonce, {
			accounts: {
				topology,
				multisig: multisig.publicKey,
				payer: program.provider.wallet.publicKey,
				membership,
				multisigProgram: multisigProgram.programId,
				systemProgram
			},
			instructions: [
				await multisigProgram.account.multisig.createInstruction(
					multisig,
					200)
			],
			signers: [multisig],
		});


		let multisigData = await multisigProgram.account.multisig.fetch(multisig.publicKey);
		assert.equal(multisigData.nonce, nonce);

		const membershipData = await program.account.membership.fetch(membership);
		assert.equal(membershipData.username, username);

		let topologyData = await program.account.topology.fetch(topology);
		assert.equal(topologyData.authority.toBase58(), signer.toBase58());

		// create 'update topology' -------------------------------------------------------
		let transaction = Keypair.generate();

		let updateTopologyIxData = { id: 0, value: 1 };
		let data = program.coder.instruction.encode("update_topology", updateTopologyIxData);

		let accounts = program.instruction.updateTopology.accounts({
			topology,
			authority: signer
		}) as ITransactionAccount[];

		let pid = program.programId;

		const transactionSize = 1000;

		await program.rpc.createTransaction(pid, accounts, data, {
			accounts: {
				membership,
				multisig: multisig.publicKey,
				transaction: transaction.publicKey,
				multisigProgram: multisigProgram.programId,
				systemProgram
			},
			signers: [transaction],
			instructions: [
				await multisigProgram.account.transaction.createInstruction(
					transaction,
					transactionSize
				)
			]
		});

		let txData = await multisigProgram.account.transaction.fetch(transaction.publicKey);

		const ixBorshCoder = program.coder.instruction as BorshInstructionCoder;
		const ix = ixBorshCoder.decode(txData.data as Buffer);
		assert.equal(ix.name, 'updateTopology');
		assert.deepEqual(ix.data, updateTopologyIxData);

		// execute -------------------------------------------------------

		let remainingAccounts = accounts
			.map(a => a.pubkey.equals(signer) ? { ...a, isSigner: false } : a)
			.concat({
				pubkey: program.programId,
				isSigner: false,
				isWritable: false
			});

		await multisigProgram.rpc.executeTransaction({
			accounts: {
				multisig: multisig.publicKey,
				multisigSigner: signer,
				transaction: transaction.publicKey
			},
			remainingAccounts
		});

		transaction = Keypair.generate();

		topologyData = await program.account.topology.fetch(topology);
		assert.equal(topologyData.values[0], 1);

		// create 'set owners' -------------------------------------------------------

		multisigData = await multisigProgram.account.multisig.fetch(multisig.publicKey);
		let newOwners = multisigData.owners;

		let user = Keypair.generate();
		let userTx = await program.provider.connection.requestAirdrop(user.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
		await program.provider.connection.confirmTransaction(userTx);

		let [newMembership] = await PublicKey.findProgramAddress([Buffer.from('membership'), user.publicKey.toBytes()],
			program.programId);

		pid = multisigProgram.programId;
		transaction = Keypair.generate();

		newOwners.push(newMembership);

		data = multisigProgram.coder.instruction.encode("set_owners", {
			owners: newOwners
		});

		accounts = multisigProgram.instruction.setOwners.accounts({
			multisig: multisig.publicKey,
			multisigSigner: signer
		}) as ITransactionAccount[];

		await program.rpc.addMembershipAndCreateTransaction(`${username}_1`, user.publicKey, pid, accounts, data, {
			accounts: {
				proposer: membership,
				wallet: program.provider.wallet.publicKey,
				multisig: multisig.publicKey,
				transaction: transaction.publicKey,
				multisigProgram: multisigProgram.programId,
				membership: newMembership,
				systemProgram
			},
			signers: [transaction],
			instructions: [
				await multisigProgram.account.transaction.createInstruction(
					transaction,
					transactionSize
				)
			]
		});

		// execute -------------------------------------------------------

		remainingAccounts = accounts
			.map(a => a.pubkey.equals(signer) ? { ...a, isSigner: false } : a)
			.concat({
				pubkey: multisigProgram.programId,
				isSigner: false,
				isWritable: false
			});


		await multisigProgram.rpc.executeTransaction({
			accounts: {
				multisig: multisig.publicKey,
				multisigSigner: signer,
				transaction: transaction.publicKey
			},
			remainingAccounts
		});

		multisigData = await multisigProgram.account.multisig.fetch(multisig.publicKey);
		assert.equal(multisigData.owners.length, 2);

		// create 'set owners' (remove) -------------------------------------------------

		multisigData = await multisigProgram.account.multisig.fetch(multisig.publicKey);
		newOwners = multisigData.owners;

		let pastOwners = newOwners.filter(m => m.toBase58() !== newMembership.toBase58());

		pid = multisigProgram.programId;
		transaction = Keypair.generate();

		data = multisigProgram.coder.instruction.encode("set_owners", {
			owners: pastOwners
		});

		accounts = multisigProgram.instruction.setOwners.accounts({
			multisig: multisig.publicKey,
			multisigSigner: signer
		}) as ITransactionAccount[];

		await program.rpc.deleteMembershipAndCreateTransaction(pid, accounts, data, {
			accounts: {
				proposer: membership,
				wallet: program.provider.wallet.publicKey,
				multisig: multisig.publicKey,
				transaction: transaction.publicKey,
				multisigProgram: multisigProgram.programId,
				membership: newMembership,
				user: user.publicKey,
				systemProgram
			},
			signers: [transaction],
			instructions: [
				await multisigProgram.account.transaction.createInstruction(
					transaction,
					transactionSize
				)
			]
		});

		// execute ----------------------------------------------------------------------
		remainingAccounts = accounts
			.map(a => a.pubkey.equals(signer) ? { ...a, isSigner: false } : a)
			.concat({
				pubkey: multisigProgram.programId,
				isSigner: false,
				isWritable: false
			});


		await multisigProgram.rpc.executeTransaction({
			accounts: {
				multisig: multisig.publicKey,
				multisigSigner: signer,
				transaction: transaction.publicKey
			},
			remainingAccounts
		});

		multisigData = await multisigProgram.account.multisig.fetch(multisig.publicKey);
		assert.equal(multisigData.owners.length, 1);

		// create 'set owners' (again) --------------------------------------------------

		multisigData = await multisigProgram.account.multisig.fetch(multisig.publicKey);
		newOwners = multisigData.owners;

		user = Keypair.generate();
		userTx = await program.provider.connection.requestAirdrop(user.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
		await program.provider.connection.confirmTransaction(userTx);

		[newMembership] = await PublicKey.findProgramAddress([Buffer.from('membership'), user.publicKey.toBytes()],
			program.programId);

		pid = multisigProgram.programId;
		transaction = Keypair.generate();

		newOwners.push(newMembership);

		data = multisigProgram.coder.instruction.encode("set_owners", {
			owners: newOwners
		});

		accounts = multisigProgram.instruction.setOwners.accounts({
			multisig: multisig.publicKey,
			multisigSigner: signer
		}) as ITransactionAccount[];

		await program.rpc.addMembershipAndCreateTransaction(`${username}_1`, user.publicKey, pid, accounts, data, {
			accounts: {
				proposer: membership,
				wallet: program.provider.wallet.publicKey,
				multisig: multisig.publicKey,
				transaction: transaction.publicKey,
				multisigProgram: multisigProgram.programId,
				membership: newMembership,
				systemProgram
			},
			signers: [transaction],
			instructions: [
				await multisigProgram.account.transaction.createInstruction(
					transaction,
					transactionSize
				)
			]
		});

		// execute -------------------------------------------------------

		remainingAccounts = accounts
			.map(a => a.pubkey.equals(signer) ? { ...a, isSigner: false } : a)
			.concat({
				pubkey: multisigProgram.programId,
				isSigner: false,
				isWritable: false
			});


		await multisigProgram.rpc.executeTransaction({
			accounts: {
				multisig: multisig.publicKey,
				multisigSigner: signer,
				transaction: transaction.publicKey
			},
			remainingAccounts
		});

		multisigData = await multisigProgram.account.multisig.fetch(multisig.publicKey);
		assert.equal(multisigData.owners.length, 2);

		// create 'create treasury' ----------------------------------------------

		const treasuryName = "main";
		const [treasury] = await PublicKey.findProgramAddress([Buffer.from('treasury'), multisig.publicKey.toBytes(), Buffer.from(treasuryName)], program.programId);

		await program.rpc.createTreasury(treasuryName, signer, {
			accounts: {
				treasury,
				payer: program.provider.wallet.publicKey,
				multisig: multisig.publicKey,
				systemProgram
			},
		});

		const treasuryData = await program.account.treasury.fetch(treasury);

		assert.equal(treasuryData.authority.toBase58(), signer.toBase58());

		// send some SOL
		let tx = new anchor.web3.Transaction().add(
			anchor.web3.SystemProgram.transfer({
				fromPubkey: program.provider.wallet.publicKey,
				toPubkey: treasury,
				lamports: 1e9 * 10
			})
		);

		tx.feePayer = program.provider.wallet.publicKey;
		await program.provider.send(tx);

		// // send back some SOL

		transaction = Keypair.generate();

		data = program.coder.instruction.encode("spend", {
			amount: new anchor.BN(5 * LAMPORTS_PER_SOL)
		});

		accounts = program.instruction.spend.accounts({
			treasury,
			authority: signer,
			to: program.provider.wallet.publicKey
		}) as ITransactionAccount[];

		pid = program.programId;

		await program.rpc.createTransaction(pid, accounts, data, {
			accounts: {
				membership,
				multisig: multisig.publicKey,
				transaction: transaction.publicKey,
				multisigProgram: multisigProgram.programId,
				systemProgram
			},
			signers: [transaction],
			instructions: [
				await multisigProgram.account.transaction.createInstruction(
					transaction,
					transactionSize
				)
			]
		});
		
		// execute --------------------------------------------------------------

		remainingAccounts = accounts
			.map(a => a.pubkey.equals(signer) ? { ...a, isSigner: false } : a)
			.concat({
				pubkey: program.programId,
				isSigner: false,
				isWritable: false
			});

		await multisigProgram.rpc.executeTransaction({
			accounts: {
				multisig: multisig.publicKey,
				multisigSigner: signer,
				transaction: transaction.publicKey
			},
			remainingAccounts
		});

		// create 'set threshold' -------------------------------------------------------

		transaction = Keypair.generate();

		data = multisigProgram.coder.instruction.encode("change_threshold", {
			threshold: new anchor.BN(2)
		});

		accounts = multisigProgram.instruction.changeThreshold.accounts({
			multisig: multisig.publicKey,
			multisigSigner: signer
		}) as ITransactionAccount[];

		pid = multisigProgram.programId;

		await program.rpc.createTransaction(pid, accounts, data, {
			accounts: {
				membership,
				multisig: multisig.publicKey,
				transaction: transaction.publicKey,
				multisigProgram: multisigProgram.programId,
				systemProgram
			},
			signers: [transaction],
			instructions: [
				await multisigProgram.account.transaction.createInstruction(
					transaction,
					transactionSize
				)
			]
		});

		// execute -------------------------------------------------------

		remainingAccounts = accounts
			.map(a => a.pubkey.equals(signer) ? { ...a, isSigner: false } : a)
			.concat({
				pubkey: multisigProgram.programId,
				isSigner: false,
				isWritable: false
			});

		await multisigProgram.rpc.executeTransaction({
			accounts: {
				multisig: multisig.publicKey,
				multisigSigner: signer,
				transaction: transaction.publicKey
			},
			remainingAccounts
		});

		// create 'update topology' -------------------------------------------------------

		transaction = Keypair.generate();

		updateTopologyIxData.id = 1;
		data = program.coder.instruction.encode("update_topology", updateTopologyIxData);

		accounts = program.instruction.updateTopology.accounts({
			topology,
			authority: signer
		}) as ITransactionAccount[];

		pid = program.programId;

		await program.rpc.createTransaction(pid, accounts, data, {
			accounts: {
				membership,
				multisig: multisig.publicKey,
				transaction: transaction.publicKey,
				multisigProgram: multisigProgram.programId,
				systemProgram
			},
			signers: [transaction],
			instructions: [
				await multisigProgram.account.transaction.createInstruction(
					transaction,
					transactionSize
				)
			]
		});

		txData = await multisigProgram.account.transaction.fetch(transaction.publicKey);

		const nix = ixBorshCoder.decode(txData.data as Buffer);
		assert.equal(nix.name, 'updateTopology');
		assert.deepEqual(nix.data, updateTopologyIxData);

		// approve ------------------------------------------------

		await program.rpc.approve({
			accounts: {
				multisig: multisig.publicKey,
				transaction: transaction.publicKey,
				wallet: user.publicKey,
				membership: newMembership,
				multisigProgram: multisigProgram.programId,
			},
			signers: [user]
		});

		// execute -------------------------------------------------------

		remainingAccounts = accounts
			.map(a => a.pubkey.equals(signer) ? { ...a, isSigner: false } : a)
			.concat({
				pubkey: program.programId,
				isSigner: false,
				isWritable: false
			});

		await multisigProgram.rpc.executeTransaction({
			accounts: {
				multisig: multisig.publicKey,
				multisigSigner: signer,
				transaction: transaction.publicKey
			},
			remainingAccounts
		});

		topologyData = await program.account.topology.fetch(topology);
		assert.equal(topologyData.values[1], 1);


	});

});
