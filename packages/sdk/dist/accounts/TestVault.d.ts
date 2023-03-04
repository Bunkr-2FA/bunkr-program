/// <reference types="node" />
import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';
import * as beetSolana from '@metaplex-foundation/beet-solana';
export type TestVaultArgs = {
    myData: string;
};
export declare const testVaultDiscriminator: number[];
export declare class TestVault implements TestVaultArgs {
    readonly myData: string;
    private constructor();
    static fromArgs(args: TestVaultArgs): TestVault;
    static fromAccountInfo(accountInfo: web3.AccountInfo<Buffer>, offset?: number): [TestVault, number];
    static fromAccountAddress(connection: web3.Connection, address: web3.PublicKey, commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig): Promise<TestVault>;
    static gpaBuilder(programId?: web3.PublicKey): beetSolana.GpaBuilder<TestVaultArgs & {
        accountDiscriminator: number[];
    }>;
    static deserialize(buf: Buffer, offset?: number): [TestVault, number];
    serialize(): [Buffer, number];
    static byteSize(args: TestVaultArgs): number;
    static getMinimumBalanceForRentExemption(args: TestVaultArgs, connection: web3.Connection, commitment?: web3.Commitment): Promise<number>;
    pretty(): {
        myData: string;
    };
}
export declare const testVaultBeet: beet.FixableBeetStruct<TestVault, TestVaultArgs & {
    accountDiscriminator: number[];
}>;
