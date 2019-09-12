const pkg = require('../pkg/miniscript_js');
const Miniscript = require('./miniscript');
// console.log(miniscript.to_miniscript('or(pk(A),pk(B))'));
// console.log(miniscript.policy_to_miniscript('or(pk(A),pk(B))'));
// console.log(miniscript.policy_to_script('and(pk(022222222222222222222222222222222222222222222222222222222222222222),pk(022222222222222222222222222222222222222222222222222222222222222222))'));

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
  // miniscript: parseMiniscript,
  // script: parseScript,
};

// console.log(miniscript.script_size(
//   'and(pk(022222222222222222222222222222222222222222222222222222222222222222),pk(022222222222222222222222222222222222222222222222222222222222222222))',
//   'POLICY',
//   'PUBLIC_KEY'
// ));
