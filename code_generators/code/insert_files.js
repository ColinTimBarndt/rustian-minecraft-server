const fs = require("fs");
const path = require("path");

/**
 * @type {{[source: string]: string}}
 */
const destinations = JSON.parse(fs.readFileSync("./destinations.json"));

Object.entries(destinations).forEach(([source, dest]) => {
  source = path.join("../generated_code", source);
  dest = path.join("../../src", dest);
  fs.copyFile(source, dest, (err) => {
    if (err) {
      console.error(`Failed to copy file(s) from "${source}" to "${dest}":`);
      console.error(err);
    } else {
      console.info(`Copied file(s) from "${source}" to "${dest}"`);
    }
  });
});
