/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  publicKey,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  mapSerializer,
  string,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type UpdateXmintMetadataInstructionAccounts = {
  poolManager: PublicKey | Pda;
  owner: Signer;
  metadataAccount: PublicKey | Pda;
  rent?: PublicKey | Pda;
  tokenMetadataProgram?: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
};

// Data.
export type UpdateXmintMetadataInstructionData = {
  discriminator: Array<number>;
  name: string;
  symbol: string;
  uri: string;
};

export type UpdateXmintMetadataInstructionDataArgs = {
  name: string;
  symbol: string;
  uri: string;
};

export function getUpdateXmintMetadataInstructionDataSerializer(): Serializer<
  UpdateXmintMetadataInstructionDataArgs,
  UpdateXmintMetadataInstructionData
> {
  return mapSerializer<
    UpdateXmintMetadataInstructionDataArgs,
    any,
    UpdateXmintMetadataInstructionData
  >(
    struct<UpdateXmintMetadataInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['name', string()],
        ['symbol', string()],
        ['uri', string()],
      ],
      { description: 'UpdateXmintMetadataInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [106, 232, 122, 160, 39, 26, 40, 131],
    })
  ) as Serializer<
    UpdateXmintMetadataInstructionDataArgs,
    UpdateXmintMetadataInstructionData
  >;
}

// Args.
export type UpdateXmintMetadataInstructionArgs =
  UpdateXmintMetadataInstructionDataArgs;

// Instruction.
export function updateXmintMetadata(
  context: Pick<Context, 'programs'>,
  input: UpdateXmintMetadataInstructionAccounts &
    UpdateXmintMetadataInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'soldStaking',
    'EuhcfekB1smmCcNqr38FvXtmWGkDy3rx8u9L1hf7ee3E'
  );

  // Accounts.
  const resolvedAccounts = {
    poolManager: {
      index: 0,
      isWritable: true as boolean,
      value: input.poolManager ?? null,
    },
    owner: {
      index: 1,
      isWritable: false as boolean,
      value: input.owner ?? null,
    },
    metadataAccount: {
      index: 2,
      isWritable: true as boolean,
      value: input.metadataAccount ?? null,
    },
    rent: { index: 3, isWritable: false as boolean, value: input.rent ?? null },
    tokenMetadataProgram: {
      index: 4,
      isWritable: false as boolean,
      value: input.tokenMetadataProgram ?? null,
    },
    systemProgram: {
      index: 5,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: UpdateXmintMetadataInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.rent.value) {
    resolvedAccounts.rent.value = publicKey(
      'SysvarRent111111111111111111111111111111111'
    );
  }
  if (!resolvedAccounts.tokenMetadataProgram.value) {
    resolvedAccounts.tokenMetadataProgram.value = context.programs.getPublicKey(
      'mplTokenMetadata',
      'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
    );
    resolvedAccounts.tokenMetadataProgram.isWritable = false;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getUpdateXmintMetadataInstructionDataSerializer().serialize(
    resolvedArgs as UpdateXmintMetadataInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
