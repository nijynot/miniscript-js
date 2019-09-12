const { ScriptType, PublicKeyType } = require('./utils/common');

const miniscript = require('./methods/miniscript');
const script = require('./methods/script');
const semantic = require('./methods/semantic');

class Miniscript {
  constructor(options) {
    let optionsCopy = {};

    if (typeof options === 'object') {
      optionsCopy = Object.assign({}, options);
    }

    this.options = {};
    this.options.miniscript = optionsCopy.miniscript;
    this.options.scriptType = ScriptType.Miniscript;
    this.options.pkType = optionsCopy.pkType;

    if (!Object.values(PublicKeyType).includes(this.options.pkType)) {
      throw new Error(`Invalid \`pkType\`, ${this.options.pkType}`);
    }

    this.miniscript = {};
    this.miniscript.encode = this._miniscriptEncode.bind(this);
    this.miniscript.scriptSize = this._scriptSize.bind(this);
    this.miniscript.maxSatisfactionWitnessElements =
      this._maxSatisfactionWitnessElements.bind(this);
    this.miniscript.maxSatisfactionSize = this._maxSatisfactionSize.bind(this);

    this.script = {};
    this.script.encode = this._scriptEncode.bind(this);
    this.script.len = this._len.bind(this);
    this.script.isEmpty = this._isEmpty.bind(this);
    this.script.toBytes = this._toBytes.bind(this);
    this.script.toP2SH = this._toP2SH.bind(this);
    this.script.toV0P2WSH = this._toV0P2WSH.bind(this);
    this.script.isP2SH = this._isP2SH.bind(this);
    this.script.isP2PKH = this._isP2PKH.bind(this);
    this.script.isP2PK = this._isP2PK.bind(this);
    this.script.isWitnessProgram = this._isWitnessProgram.bind(this);
    this.script.isV0P2WSH = this._isV0P2WSH.bind(this);
    this.script.isV0P2WPKH = this._isV0P2WPKH.bind(this);
    this.script.isOPRETURN = this._isOPRETURN.bind(this);
    this.script.isProvablyUnspendable = this._isProvablyUnspendable.bind(this);

    this.semantic = {};
    this.semantic.normalized = this._normalized.bind(this);
    this.semantic.isTrivial = this._isTrivial.bind(this);
    this.semantic.isUnsatisfiable = this._isUnsatisfiable.bind(this);
    this.semantic.relativeTimelocks = this._relativeTimelocks.bind(this);
    this.semantic.atAge = this._atAge.bind(this);
    this.semantic.nKeys = this._nKeys.bind(this);
    this.semantic.minimumNKeys = this._minimumNKeys.bind(this);
    this.semantic.sorted = this._sorted.bind(this);
  }

  /**
   * Miniscript
   */
  _miniscriptEncode() {
    return this.options.miniscript;
  }

  _scriptSize() {
    return miniscript.scriptSize(this.options);
  }

  _maxSatisfactionWitnessElements() {
    return miniscript.maxSatisfactionWitnessElements(this.options);
  }

  _maxSatisfactionSize() {
    return miniscript.maxSatisfactionSize(this.options);
  }

  /**
   * Script
   */
  _scriptEncode() {
    return script.encode(this.options);
  }

  _len() {
    return script.len(this.options);
  }

  _isEmpty() {
    return script.isEmpty(this.options);
  }

  _toBytes() {
    return script.toBytes(this.options);
  }

  _toP2SH() {
    return script.toP2SH(this.options);
  }

  _toV0P2WSH() {
    return script.toV0P2WSH(this.options);
  }

  _isP2SH() {
    return script.isP2SH(this.options);
  }

  _isP2PKH() {
    return script.isP2PKH(this.options);
  }

  _isP2PK() {
    return script.isP2PK(this.options);
  }

  _isWitnessProgram() {
    return script.isWitnessProgram(this.options);
  }

  _isV0P2WSH() {
    return script.isV0P2WSH(this.options);
  }

  _isV0P2WSH() {
    return script.isV0P2WSH(this.options);
  }

  _isV0P2WPKH() {
    return script.isV0P2WPKH(this.options);
  }

  _isOPRETURN() {
    return script.isOPRETURN(this.options);
  }

  _isProvablyUnspendable() {
    return script.isProvablyUnspendable(this.options);
  }

  /**
   * Semantic
   */
  _normalized() {
    return semantic.normalized(this.options);
  }

  _isTrivial() {
    return semantic.isTrivial(this.options);
  }

  _isUnsatisfiable() {
    return semantic.isUnsatisfiable(this.options);
  }

  _relativeTimelocks() {
    return semantic.relativeTimelocks(this.options);
  }

  _atAge(time) {
    return semantic.atAge(this.options, time);
  }

  _nKeys() {
    return semantic.nKeys(this.options);
  }

  _minimumNKeys() {
    return semantic.minimumNKeys(this.options);
  }

  _sorted() {
    return semantic.sorted(this.options);
  }
}

module.exports = Miniscript;
