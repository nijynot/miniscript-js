const pkg = require('../pkg/miniscript_js');
const Miniscript = require('./miniscript');

function parsePolicy(script, pkType) {
  const ms = pkg.policy_to_miniscript(script);
  return new Miniscript({
    miniscript: ms,
    scriptType: 'MINISCRIPT',
    pkType,
  });
}

module.exports = {
  parsePolicy,
};
