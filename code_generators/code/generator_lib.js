const fs = require("fs");

/**
 * @abstract
 */
class CodeGenerator {
  constructor() {
    this.finished = false;
  }

  /**
   * Finishes this generator and prevents all further writing
   */
  finish() {
    throw new Error("Not implemented!");
  }

  /**
   * Write a text to this generator
   * @param {string} text
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  write(text) {
    throw new Error("Not implemented!");
  }

  /**
   * Writes a new line
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  newLine() {
    this.write("\n");
    return this;
  }

  /**
   * Write spaces
   * @param {number} [indent=0] - indentation in spaces
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  indent(indent = 0) {
    this.write(" ".repeat(indent));
    return this;
  }

  /**
   * Write a line followed by a new line character with optional indentation
   * @param {string[]} text One line or multiple lines separated by new lines
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  writeLine(...text) {
    text.forEach((text) => {
      text.split(/\r?\n/g).forEach((line) => {
        this.indent().write(line).newLine();
      });
    });
    return this;
  }

  /**
   * Write a line comment. Additional lines are separated by new lines
   * @param {string[]} text - Comment
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  commentLine(...text) {
    text.forEach((text) => {
      text.split(/\r?\n/g).forEach((line) => {
        this.writeLine(`// ${line}`);
      });
    });
    return this;
  }

  /**
   * Write a line comment. Additional lines are separated by new lines
   * @param {string[]} text - Comment
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  documentLine(...text) {
    text.forEach((text) => {
      text.split(/\r?\n/g).forEach((line) => {
        this.writeLine(`/// ${line}`);
      });
    });
    return this;
  }
}

class RustCodeGenerator extends CodeGenerator {
  /**
   * Create a new code generator
   * @param {string} file
   */
  constructor(file) {
    super();
    if (!file.endsWith(".rs")) file += ".rs";
    this.writer = fs
      .createWriteStream(file, { flags: "w" })
      .on("finish", () => {
        console.info("Writing operation finished");
      })
      .on("error", (err) => {
        console.warn("Error:");
        console.warn(err.stack);
      });
  }

  /**
   * Finishes this generator and prevents all further writing
   */
  finish() {
    if (!this.finished) {
      this.writer.close();
      this.finished = true;
    }
  }

  /**
   * Write a text to the file
   * @param {string} text
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  write(text) {
    if (this.finished) {
      throw new Error("This generator has already finished writing");
    }
    this.writer.write(text);
    return this;
  }
}

class FunctionGenerator extends CodeGenerator {
  /**
   * @param {CodeGenerator} gen - code generator to use
   * @param {string} name - function name
   * @param {string} [retType=null] - return type of the function
   * @param {boolean} [pub=false] - whether this function is public or not
   */
  constructor(gen, name, retType = null, pub = false) {
    super();
    this.generator = gen;
    this.params = [];
    this.returnType = retType;
    gen.indent().write(`${pub ? "pub " : ""}fn ${name}(`);
  }

  /**
   * Finishes this generator and prevents all further writing
   */
  finish() {
    if (!this.finished) {
      if (this.params) {
        this.finishParams();
      }
      this.generator.writeLine("}");
      this.finished = true;
    }
  }

  /**
   * Write a text to the generator
   * @param {string} text
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  write(text) {
    if (this.finished) {
      throw new Error("This generator has already finished writing");
    }
    if (this.params) {
      this.finishParams();
    }
    this.generator.write(text);
    return this;
  }

  indent(indent = 0) {
    if (this.params) {
      this.finishParams();
    }
    this.generator.indent(indent + 2);
    return this;
  }

  /**
   * @throws if this generator has already finished writing params
   * @throws if this generator is already finished
   */
  finishParams() {
    if (this.finished) {
      throw new Error("This generator has already finished writing");
    }
    if (this.params) {
      this.generator
        .write(
          `${this.params.join(", ")})${
            this.returnType ? ` -> ${this.returnType}` : ""
          } {`
        )
        .newLine();
      this.params = null;
    } else {
      throw new Error("Params already finished");
    }
  }

  /**
   * @typedef {(name: string, type: string=) => AddParamFunction} AddParamFunction
   */

  /**
   * Add a parameter to this function
   * @param {string} name
   * @param {string=} type
   * @returns {AddParamFunction} this function
   */
  addParam(name, type) {
    if (this.params) this.params.push(type ? `${name}: ${type}` : name);
    return (name, type) => this.addParam(name, type);
  }
}

class CodeBlockGenerator extends CodeGenerator {
  /**
   * @param {CodeGenerator} gen - code generator to use
   */
  constructor(gen) {
    super();
    this.generator = gen;
    gen.writeLine(`{`);
  }

  /**
   * Finishes this generator and prevents all further writing
   */
  finish() {
    if (!this.finished) {
      if (this.generator instanceof InlinedGenerator)
        this.generator.indent().write("}");
      else this.generator.writeLine("}");
      this.finished = true;
    }
  }

  /**
   * Write a text to the generator
   * @param {string} text
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  write(text) {
    if (this.finished) {
      throw new Error("This generator has already finished writing");
    }
    this.generator.write(text);
    return this;
  }

  indent(indent = 0) {
    this.generator.indent(indent + 2);
    return this;
  }
}

/**
 * Does nothing special except styling code generated inside it inline
 */
class InlinedGenerator extends CodeGenerator {
  constructor(gen) {
    super();
    this.generator = gen;
    this.firstLine = true;
  }
  write(text) {
    if (this.finished) {
      throw new Error("This generator has already finished writing");
    }
    this.generator.write(text);
    return this;
  }
  finish() {
    this.finished = true;
  }
  indent(indent = 0) {
    if (this.firstLine) {
      this.firstLine = false;
    } else {
      this.generator.indent(indent);
    }
    return this;
  }
}

class MatchGenerator extends CodeGenerator {
  /**
   * @param {CodeGenerator} gen - code generator to use
   * @param {string} on - variable to match on
   */
  constructor(gen, on) {
    super();
    this.generator = gen;
    gen.writeLine(`match ${on} {`);
  }

  /**
   * Finishes this generator and prevents all further writing
   */
  finish() {
    if (!this.finished) {
      if (this.generator instanceof InlinedGenerator)
        this.generator.indent().write("}");
      else this.generator.writeLine("}");
      this.finished = true;
    }
  }

  /**
   * Write a text to the generator
   * @param {string} text
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  write(text) {
    if (this.finished) {
      throw new Error("This generator has already finished writing");
    }
    this.generator.write(text);
    return this;
  }

  indent(indent = 0) {
    this.generator.indent(indent + 2);
    return this;
  }

  /**
   * Creates a new match arm
   * @param {string} pattern - pattern to match on
   * @param {string|((gen: CodeGenerator) => void)} content - content of the match arm. `string` if inline, `(CodeGenerator) => void` if multiline.
   */
  addMatchArm(pattern, content) {
    this.indent().write(pattern).write(" => ");
    if (typeof content === "string") {
      this.write(content).write(",").newLine();
    } else {
      let gen = new InlinedGenerator(this);
      content(gen);
      gen.finish();
      this.write(",").newLine();
    }
  }
}

class EnumGenerator extends CodeGenerator {
  /**
   * @param {CodeGenerator} gen - code generator to use
   * @param {string} name - enum name
   * @param {number} [indent=0] - indentation in spaces
   * @param {boolean} [pub=false] - whether this enum is public or not
   */
  constructor(gen, name, pub = false) {
    super();
    this.generator = gen;
    gen.writeLine(`${pub ? "pub " : ""}enum ${name} {`);
  }

  /**
   * Write a text to the generator
   * @param {string} text
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  write(text) {
    if (this.finished) {
      throw new Error("This generator has already finished writing");
    }
    this.generator.write(text);
    return this;
  }

  /**
   * Finishes this generator and prevents all further writing
   */
  finish() {
    if (!this.finished) {
      if (this.generator instanceof InlinedGenerator)
        this.generator.indent().write("}");
      else this.generator.writeLine("}");
      this.finished = true;
    }
  }

  indent(indent = 0) {
    this.generator.indent(indent + 2);
    return this;
  }

  /**
   *
   * @param {string} name - Name of the enum value
   * @param {null|string[]|{[attribute: string]: string}|number} [attrs=null] - Additional attributes (tuple, struct or enum value)
   */
  addEnumValue(name, attrs = null) {
    this.indent().write(name);
    if (attrs !== null) {
      if (typeof attrs === "number") {
        this.write(" = " + attrs);
      } else if (attrs instanceof Array) {
        this.write("(");
        this.write(attrs.join(", "));
        this.write(")");
      } else if (attrs instanceof Object) {
        this.write("{").newLine();
        Object.entries().forEach((name, type) => {
          this.indent(2)
            .write(`${escapeName(name)}: ${type};`)
            .newLine();
        });
        this.write("}");
      } else {
        throw new Error("Illegal attributes argument");
      }
    }
    this.write(",").newLine();
  }
}

class ImplGenerator extends CodeGenerator {
  /**
   * @param {CodeGenerator} gen - code generator to use
   * @param {string} structName - struct, enum or tuple name
   * @param {number} [indent=0] - indentation in spaces
   * @param {boolean|string} [pubOrTrait=false] - whether this enum is public or not OR name of the trait if a trait is implemented
   */
  constructor(gen, structName, pubOrTrait = false) {
    super();
    this.generator = gen;
    gen.indent();
    if (pubOrTrait === true) gen.write("pub ");
    gen.write("impl ");
    if (typeof pubOrTrait === "string") {
      gen.write(`${pubOrTrait} for `);
    }
    gen.write(`${structName} {`).newLine();
  }

  /**
   * Write a text to the generator
   * @param {string} text
   * @returns {this} this generator
   * @throws if this generator is already finished
   */
  write(text) {
    if (this.finished) {
      throw new Error("This generator has already finished writing");
    }
    this.generator.write(text);
    return this;
  }

  /**
   * Finishes this generator and prevents all further writing
   */
  finish() {
    if (!this.finished) {
      this.generator.writeLine("}");
      this.finished = true;
    }
  }

  indent(indent = 0) {
    this.generator.indent(indent + 2);
    return this;
  }
}

/**
 * @param {CodeGenerator} gen
 * @param {string} name - name of the structure
 * @param {{[name: string]: string}} attributes - attributes and their types
 * @param {boolean} [pub=false] - whether this struct is public or not
 * @param {string[]} [pubAttrs=] - which attributes are public
 */
function writeStruct(gen, name, attributes, pub = false, pubAttrs = []) {
  gen.writeLine(`${pub ? "pub " : ""}struct ${name} {`);
  Object.entries(attributes).forEach(([name, type]) => {
    gen
      .indent(2)
      .write(
        `${pubAttrs.includes(name) ? "pub " : ""}${escapeName(name)}: ${type},`
      )
      .newLine();
  });
  gen.writeLine("}");
}

/**
 * @param {string} name - Name to escape
 * @returns {string}
 */
function escapeName(name) {
  if (name === "type") {
    name = "r#" + name;
  }
  return name;
}

class NamespacedKey {
  /**
   * @param {string} namespace
   * @param {string} id
   */
  constructor(namespace, id) {
    this.namespace = namespace;
    this.id = id;
  }
  toString() {
    return `${this.namespace}:${this.id}`;
  }
  /**
   * Converts this key to Rust code
   * @returns {string}
   */
  toRust() {
    return `NamespacedKey::new("${this.namespace}", "${this.id}")`;
  }
  /**
   * @param {string} str - namespaced id string (`namespace:id`)
   */
  static fromString(str) {
    let [namespace, id] = str.split(":");
    return new this(namespace, id);
  }
}

/**
 * Converts a string from snake_case to CamelCase
 * @param {string} str - string to convert
 * @param {boolean} big - whether the first letter should be upper case
 * @returns {string}
 */
function toCamelCase(str, big = true) {
  let res = "";
  for (let char of str) {
    if (big) {
      res += char.toUpperCase();
      big = false;
    } else if (char == "_") {
      big = true;
    } else {
      res += char.toLowerCase();
      if (!/^\w$/.test(char)) {
        big = true;
      }
    }
  }
  return res;
}

/**
 * Converts a string from CamelCase to snake_case
 * @param {string} str - string to convert
 * @returns {string}
 */
function toSnakeCase(str) {
  // In case the first letter is UpperCase, don't add an underscore
  let res = str[0].toLowerCase();
  for (let char of str.substring(1)) {
    if (/^[A-Z]$/.test(char)) {
      // UPPER CASE
      res += "_" + char.toLowerCase();
    } else {
      // lower case
      res += char;
    }
  }
  return res;
}

module.exports = {
  CodeGenerator,
  RustCodeGenerator,
  FunctionGenerator,
  CodeBlockGenerator,
  MatchGenerator,
  EnumGenerator,
  ImplGenerator,
  InlinedGenerator,
  writeStruct,
  escapeName,
  toCamelCase,
  toSnakeCase,
  NamespacedKey,
};
