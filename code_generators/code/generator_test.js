const {
  RustCodeGenerator,
  FunctionGenerator,
  CodeBlockGenerator,
  MatchGenerator,
} = require("./generator_lib");

let testFile = new RustCodeGenerator("./test.rs");

let fn = new FunctionGenerator(testFile, "testFn", true);
fn.addParam("a", "u8")("b", "i32");
fn.commentLine("Do something!");
fn.finish();

let block = new CodeBlockGenerator(testFile);
block.commentLine("Code block");
block.finish();

let match = new MatchGenerator(testFile, "5");
match.addMatchArm("3..8", 'println!("Hello world!")');
match.addMatchArm("9", (gen) => {
  let block = new CodeBlockGenerator(gen, false);
  block.commentLine("Code block");
  block.finish();
});
match.finish();

testFile.finish();
