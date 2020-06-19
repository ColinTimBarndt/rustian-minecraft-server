const lib = require("./generator_lib");

/**
 * @type {{[algorhitmName: string]: {js: (str: string) => number, rs: string[]}}}
 */
const algorhitms = {
  addChars: {
    js(str) {
      let sum = 0;
      [...str].forEach((c) => (sum = (sum + c.charCodeAt(0)) & 0xffff));
      return sum;
    },
    rs: [
      "let mut sum = 0u16;",
      "input.as_bytes().iter().for_each(|c| sum = sum.overflowing_add(*c as u16).0);",
      "sum",
    ],
  },
};

{
  for (let i = 1; i < 16; i++) {
    for (let factor = 1; factor < 256; factor++) {
      algorhitms[`rotXorShift${i}Factor${factor}`] = {
        js(str) {
          let sum = 0;
          [...str].forEach((c) => {
            let char = c.charCodeAt(0);
            sum = rotateLeft(sum, i) ^ (char * factor);
          });
          return sum;
        },
        rs: [
          "use std::ops::BitXor;",
          "let mut sum = 0u16;",
          `input.as_bytes().iter().for_each(|c| sum = sum.rotate_left(${i}).bitxor(*c as u16 * ${factor}u16));`,
          "sum",
        ],
      };
    }
  }
}

//console.log(algorhitms);

/**
 * Finds the best hashing algorhitm for the given inputs
 * @param {string[]} input
 */
function findAlgorhitmFor(input) {
  /**
   * @type {Map<number, string>}
   */
  let results = new Map();
  for (let algoName in algorhitms) {
    let algo = algorhitms[algoName];
    let score = testAlgorhitm(algo.js, input);
    if (score >= 0) {
      results.set(score, algoName);
    }
  }
  if (results.size === 0) return null;
  return [...results.entries()].sort((a, b) => a[0] - b[0])[0][1];
}

/**
 * Tests whether the given hashing algorhitm generates
 * unique hashes for the given list of inputs
 * @param {(string) => number} algo Hash algorhitm
 * @param {string[]} input input strings
 * @return {number} -1 if it has duplicates or the highest hash
 */
function testAlgorhitm(algo, input) {
  /**
   * @type {Set<number>}
   */
  let hashes = new Set();
  let max = 0;
  for (let str of input) {
    let hash = algo(str);
    if (hashes.has(hash)) return -1;
    else {
      hashes.add(hash);
      max = Math.max(max, hash);
    }
  }
  return max;
}

/**
 * @param {number} num
 * @param {number} amount
 * @returns {number}
 */
function rotateLeft(num, amount) {
  num <<= amount;
  let base = num & 0xffff;
  let overflow = num - base;
  overflow >>= 16;
  return base + overflow;
}

/**
 *
 * @param {string} name - Name of the hash algorhitm
 * @returns {(str: string) => number}
 */
function getJSImplementation(name) {
  let algo = algorhitms[name];
  return algo ? algo.js : null;
}

/**
 * @param {lib.CodeGenerator} gen - Code Generator
 * @param {string} name
 */
function writeRustFunction(gen, name) {
  let algo = algorhitms[name];
  let fn = new lib.FunctionGenerator(gen, lib.toSnakeCase(name), "u16");
  fn.addParam("input", "&String");
  fn.writeLine(...algo.rs);
  fn.finish();
}

module.exports = { findAlgorhitmFor, writeRustFunction, getJSImplementation };
