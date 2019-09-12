const pkg = require('../../pkg/miniscript_js');

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function normalized(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.normalized(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isTrivial(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_trivial(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 */
function isUnsatisfiable(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.is_unsatisfiable(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 * @returns {Object}
 */
function relativeTimelocks(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.relative_timelocks(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 * @param {number} time
 * @returns {Object}
 */
function atAge(options, time) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.at_age(miniscript, scriptType, pkType, time);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 * @returns {Object}
 */
function nKeys(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.n_keys(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 * @returns {Object}
 */
function minimumNKeys(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.minimum_n_keys(miniscript, scriptType, pkType);
}

/**
 * Encode into a Bitcoin Script.
 * @param {Object} options
 * @param {string} options.miniscript
 * @param {string} options.scriptType
 * @param {string} options.pkType
 * @returns {Object}
 */
function sorted(options) {
  const { miniscript, scriptType, pkType } = options;
  return pkg.sorted(miniscript, scriptType, pkType);
}

module.exports = {
  normalized,
  isTrivial,
  isUnsatisfiable,
  relativeTimelocks,
  atAge,
  nKeys,
  minimumNKeys,
  sorted,
};
