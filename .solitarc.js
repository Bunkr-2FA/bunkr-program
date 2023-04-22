const path = require('path');
const programDir = path.join(__dirname, 'programs', 'vault-program');
const idlDir = path.join(__dirname, 'target/idl');
const sdkDir = path.join(__dirname, 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'vault_program',
  programId: 'BunKrGBXdGxyTLjvE44eQXDuKY7TyHZfPu9bj2Ugk5j2',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};