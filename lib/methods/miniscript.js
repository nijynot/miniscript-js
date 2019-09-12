const pkg = require('../../pkg/miniscript_js');

/**
 * Get script size of a Miniscript.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function scriptSize(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.script_size(miniscript, scriptType, pkType);
}

/**
 * Get script size of a Miniscript.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function maxSatisfactionWitnessElements(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.max_satisfaction_witness_elements(miniscript, scriptType, pkType);
}

/**
 * Get script size of a Miniscript.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function maxSatisfactionSize(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.max_satisfaction_size(miniscript, scriptType, pkType);
}

module.exports = {
  scriptSize,
  maxSatisfactionWitnessElements,
  maxSatisfactionSize,
};
