"use strict";

const path = require("path");
const idl = require("./target/idl/vault_program.json");

// const accountProviders = require("/Users/orion/Documents/GitHub/vault-program/src/generated/accounts");

const accountProviders = require('./packages/sdk/dist/accounts');

const validator = {
  programs: [
    {
      label: "Vault Program",
      programId: "BunKrGBXdGxyTLjvE44eQXDuKY7TyHZfPu9bj2Ugk5j2",
      deployPath: path.resolve(__dirname, "./target/deploy/vault_program.so"),
      ledgerDir: path.resolve(__dirname, "./test-ledger"),
    },
  ]
};

// q: how do I convert this:  { TestVault } from './src/generated/accounts/TestVault'; to this: 

module.exports = {
  validator,
  relay: {
    accountProviders
  },
};

