"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.testVaultBeet = exports.TestVault = exports.testVaultDiscriminator = void 0;
const beet = __importStar(require("@metaplex-foundation/beet"));
const web3 = __importStar(require("@solana/web3.js"));
const beetSolana = __importStar(require("@metaplex-foundation/beet-solana"));
exports.testVaultDiscriminator = [20, 73, 64, 180, 6, 44, 154, 93];
class TestVault {
    constructor(myData) {
        this.myData = myData;
    }
    static fromArgs(args) {
        return new TestVault(args.myData);
    }
    static fromAccountInfo(accountInfo, offset = 0) {
        return TestVault.deserialize(accountInfo.data, offset);
    }
    static async fromAccountAddress(connection, address, commitmentOrConfig) {
        const accountInfo = await connection.getAccountInfo(address, commitmentOrConfig);
        if (accountInfo == null) {
            throw new Error(`Unable to find TestVault account at ${address}`);
        }
        return TestVault.fromAccountInfo(accountInfo, 0)[0];
    }
    static gpaBuilder(programId = new web3.PublicKey('undefined')) {
        return beetSolana.GpaBuilder.fromStruct(programId, exports.testVaultBeet);
    }
    static deserialize(buf, offset = 0) {
        return exports.testVaultBeet.deserialize(buf, offset);
    }
    serialize() {
        return exports.testVaultBeet.serialize({
            accountDiscriminator: exports.testVaultDiscriminator,
            ...this,
        });
    }
    static byteSize(args) {
        const instance = TestVault.fromArgs(args);
        return exports.testVaultBeet.toFixedFromValue({
            accountDiscriminator: exports.testVaultDiscriminator,
            ...instance,
        }).byteSize;
    }
    static async getMinimumBalanceForRentExemption(args, connection, commitment) {
        return connection.getMinimumBalanceForRentExemption(TestVault.byteSize(args), commitment);
    }
    pretty() {
        return {
            myData: this.myData,
        };
    }
}
exports.TestVault = TestVault;
exports.testVaultBeet = new beet.FixableBeetStruct([
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['myData', beet.utf8String],
], TestVault.fromArgs, 'TestVault');
//# sourceMappingURL=TestVault.js.map