const pkg = require('../pkg/miniscript_js');
const Miniscript = require('./miniscript');
const common = require('./utils/common');

const { ScriptType } = common;

function parsePolicy(script, pkType) {
  const ms = pkg.policy_to_miniscript(script);
  return new Miniscript({
    miniscript: ms,
    scriptType: ScriptType.Miniscript,
    pkType,
  });
}

function parseMiniscript(script, pkType) {
  return new Miniscript({
    miniscript: script,
    scriptType: ScriptType.Miniscript,
    pkType,
  });
}

module.exports = {
  parsePolicy,
  parseMiniscript,
  ...common,
};
