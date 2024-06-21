/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Account,
  Context,
  Pda,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  publicKey as toPublicKey,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  i64,
  mapSerializer,
  publicKey as publicKeySerializer,
  string,
  struct,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';

export type PoolManager = Account<PoolManagerAccountData>;

export type PoolManagerAccountData = {
  discriminator: Array<number>;
  bump: number;
  owner: PublicKey;
  admin: PublicKey;
  baseMint: PublicKey;
  xMint: PublicKey;
  baseMintDecimals: number;
  xMintDecimals: number;
  annualYieldRate: bigint;
  initialExchangeRate: bigint;
  lastYieldChangeExchangeRate: bigint;
  inceptionTimestamp: bigint;
  lastYieldChangeTimestamp: bigint;
  baseBalance: bigint;
  xSupply: bigint;
};

export type PoolManagerAccountDataArgs = {
  bump: number;
  owner: PublicKey;
  admin: PublicKey;
  baseMint: PublicKey;
  xMint: PublicKey;
  baseMintDecimals: number;
  xMintDecimals: number;
  annualYieldRate: number | bigint;
  initialExchangeRate: number | bigint;
  lastYieldChangeExchangeRate: number | bigint;
  inceptionTimestamp: number | bigint;
  lastYieldChangeTimestamp: number | bigint;
  baseBalance: number | bigint;
  xSupply: number | bigint;
};

export function getPoolManagerAccountDataSerializer(): Serializer<
  PoolManagerAccountDataArgs,
  PoolManagerAccountData
> {
  return mapSerializer<PoolManagerAccountDataArgs, any, PoolManagerAccountData>(
    struct<PoolManagerAccountData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['bump', u8()],
        ['owner', publicKeySerializer()],
        ['admin', publicKeySerializer()],
        ['baseMint', publicKeySerializer()],
        ['xMint', publicKeySerializer()],
        ['baseMintDecimals', u8()],
        ['xMintDecimals', u8()],
        ['annualYieldRate', u64()],
        ['initialExchangeRate', u64()],
        ['lastYieldChangeExchangeRate', u64()],
        ['inceptionTimestamp', i64()],
        ['lastYieldChangeTimestamp', i64()],
        ['baseBalance', u64()],
        ['xSupply', u64()],
      ],
      { description: 'PoolManagerAccountData' }
    ),
    (value) => ({
      ...value,
      discriminator: [54, 241, 200, 10, 177, 151, 78, 17],
    })
  ) as Serializer<PoolManagerAccountDataArgs, PoolManagerAccountData>;
}

export function deserializePoolManager(rawAccount: RpcAccount): PoolManager {
  return deserializeAccount(rawAccount, getPoolManagerAccountDataSerializer());
}

export async function fetchPoolManager(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<PoolManager> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  assertAccountExists(maybeAccount, 'PoolManager');
  return deserializePoolManager(maybeAccount);
}

export async function safeFetchPoolManager(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<PoolManager | null> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  return maybeAccount.exists ? deserializePoolManager(maybeAccount) : null;
}

export async function fetchAllPoolManager(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<PoolManager[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, 'PoolManager');
    return deserializePoolManager(maybeAccount);
  });
}

export async function safeFetchAllPoolManager(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<PoolManager[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializePoolManager(maybeAccount as RpcAccount));
}

export function getPoolManagerGpaBuilder(
  context: Pick<Context, 'rpc' | 'programs'>
) {
  const programId = context.programs.getPublicKey(
    'soldStaking',
    'EuhcfekB1smmCcNqr38FvXtmWGkDy3rx8u9L1hf7ee3E'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      discriminator: Array<number>;
      bump: number;
      owner: PublicKey;
      admin: PublicKey;
      baseMint: PublicKey;
      xMint: PublicKey;
      baseMintDecimals: number;
      xMintDecimals: number;
      annualYieldRate: number | bigint;
      initialExchangeRate: number | bigint;
      lastYieldChangeExchangeRate: number | bigint;
      inceptionTimestamp: number | bigint;
      lastYieldChangeTimestamp: number | bigint;
      baseBalance: number | bigint;
      xSupply: number | bigint;
    }>({
      discriminator: [0, array(u8(), { size: 8 })],
      bump: [8, u8()],
      owner: [9, publicKeySerializer()],
      admin: [41, publicKeySerializer()],
      baseMint: [73, publicKeySerializer()],
      xMint: [105, publicKeySerializer()],
      baseMintDecimals: [137, u8()],
      xMintDecimals: [138, u8()],
      annualYieldRate: [139, u64()],
      initialExchangeRate: [147, u64()],
      lastYieldChangeExchangeRate: [155, u64()],
      inceptionTimestamp: [163, i64()],
      lastYieldChangeTimestamp: [171, i64()],
      baseBalance: [179, u64()],
      xSupply: [187, u64()],
    })
    .deserializeUsing<PoolManager>((account) => deserializePoolManager(account))
    .whereField('discriminator', [54, 241, 200, 10, 177, 151, 78, 17]);
}

export function getPoolManagerSize(): number {
  return 195;
}

export function findPoolManagerPda(
  context: Pick<Context, 'eddsa' | 'programs'>
): Pda {
  const programId = context.programs.getPublicKey(
    'soldStaking',
    'EuhcfekB1smmCcNqr38FvXtmWGkDy3rx8u9L1hf7ee3E'
  );
  return context.eddsa.findPda(programId, [
    string({ size: 'variable' }).serialize('pool-manager'),
  ]);
}

export async function fetchPoolManagerFromSeeds(
  context: Pick<Context, 'eddsa' | 'programs' | 'rpc'>,
  options?: RpcGetAccountOptions
): Promise<PoolManager> {
  return fetchPoolManager(context, findPoolManagerPda(context), options);
}

export async function safeFetchPoolManagerFromSeeds(
  context: Pick<Context, 'eddsa' | 'programs' | 'rpc'>,
  options?: RpcGetAccountOptions
): Promise<PoolManager | null> {
  return safeFetchPoolManager(context, findPoolManagerPda(context), options);
}
