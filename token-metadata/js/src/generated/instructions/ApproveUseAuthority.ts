/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token';
import * as definedTypes from '../types';
import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';

/**
 * @category Instructions
 * @category ApproveUseAuthority
 * @category generated
 */
export type ApproveUseAuthorityInstructionArgs = {
  approveUseAuthorityArgs: definedTypes.ApproveUseAuthorityArgs;
};
/**
 * @category Instructions
 * @category ApproveUseAuthority
 * @category generated
 */
const ApproveUseAuthorityStruct = new beet.BeetArgsStruct<
  ApproveUseAuthorityInstructionArgs & {
    instructionDiscriminator: number;
  }
>(
  [
    ['instructionDiscriminator', beet.u8],
    ['approveUseAuthorityArgs', definedTypes.approveUseAuthorityArgsBeet],
  ],
  'ApproveUseAuthorityInstructionArgs',
);
/**
 * Accounts required by the _ApproveUseAuthority_ instruction
 *
 * @property [_writable_] useAuthorityRecord Use Authority Record PDA
 * @property [**signer**] owner Owner
 * @property [**signer**] payer Payer
 * @property [] user A Use Authority
 * @property [_writable_] ownerTokenAccount Owned Token Account Of Mint
 * @property [] metadata Metadata account
 * @property [] mint Mint of Metadata
 * @property [] burner Program As Signer (Burner)
 * @property [] systemAccount System program
 * @category Instructions
 * @category ApproveUseAuthority
 * @category generated
 */
export type ApproveUseAuthorityInstructionAccounts = {
  useAuthorityRecord: web3.PublicKey;
  owner: web3.PublicKey;
  payer: web3.PublicKey;
  user: web3.PublicKey;
  ownerTokenAccount: web3.PublicKey;
  metadata: web3.PublicKey;
  mint: web3.PublicKey;
  burner: web3.PublicKey;
  systemAccount: web3.PublicKey;
};

const approveUseAuthorityInstructionDiscriminator = 20;

/**
 * Creates a _ApproveUseAuthority_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category ApproveUseAuthority
 * @category generated
 */
export function createApproveUseAuthorityInstruction(
  accounts: ApproveUseAuthorityInstructionAccounts,
  args: ApproveUseAuthorityInstructionArgs,
) {
  const {
    useAuthorityRecord,
    owner,
    payer,
    user,
    ownerTokenAccount,
    metadata,
    mint,
    burner,
    systemAccount,
  } = accounts;

  const [data] = ApproveUseAuthorityStruct.serialize({
    instructionDiscriminator: approveUseAuthorityInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: useAuthorityRecord,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: owner,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: payer,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: user,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: ownerTokenAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: metadata,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: mint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: burner,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: systemAccount,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId: new web3.PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'),
    keys,
    data,
  });
  return ix;
}
