const pkg = require('../../pkg/miniscript_js');

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function encode(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.to_script(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function len(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.len(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isEmpty(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_empty(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function toBytes(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.to_bytes(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function toP2SH(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.to_p2sh(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function toV0P2WSH(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.to_v0_p2wsh(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isP2SH(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_p2sh(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isP2PKH(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_p2pkh(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isP2PK(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_p2pk(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isWitnessProgram(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_witness_program(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isV0P2WSH(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_v0_p2wsh(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isV0P2WPKH(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_v0_p2wpkh(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isOPRETURN(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_op_return(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isProvablyUnspendable(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_provably_unspendable(miniscript, scriptType, pkType);
}

module.exports = {
  encode,
  len,
  isEmpty,
  toBytes,
  toP2SH,
  toV0P2WSH,
  isP2SH,
  isP2PKH,
  isP2PK,
  isWitnessProgram,
  isV0P2WSH,
  isV0P2WPKH,
  isOPRETURN,
  isProvablyUnspendable,
};
