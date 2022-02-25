import * as anchor from '@project-serum/anchor';
import { BorshInstructionCoder, Program } from '@project-serum/anchor';
import { NamaphMultisig as Namaph } from '../target/types/namaph_multisig';
import { SerumMultisig } from '../target/types/serum_multisig';
import assert from 'assert';

describe('multisig-cpi', () => {
});

interface IAccountMeta {
	pubkey: anchor.web3.PublicKey,
	isSigner: boolean,
	isWritable: boolean
}
