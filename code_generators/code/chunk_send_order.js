/**
 * This code generates rust code that creates a list of
 * chunk positions sorted by distance to the given center
 * used for ordering chunk sending packets or chunk
 * loading.
 * This method utilizes a pre-generated sorted array of
 * chunk offsets for a better performance.
 * @see https://minecraft.gamepedia.com/Options#Video_Settings
 * @author Colin Tim Barndt
 */

const fs = require("fs");
const lib = require("./generator_lib");

const file = new lib.RustCodeGenerator("../generated_code/chunk_send_order.rs");

/**
 * Sets the maximum supported render distance of the server.
 * The client-side bounds are 2..=32
 */
const maxRenderDistance = 16;

file.commentLine("This file was automatically generated");

file.writeLine(
  "#![allow(unused)]",
  "use std::collections::HashSet;",
  "use crate::server::universe::world::chunk::ChunkPosition;",
  "",
  `pub const MAX_RENDER_DISTANCE: u8 = ${maxRenderDistance};`
);

{
  /**
   * All chunk offsets from the player chunk sorted by
   * their distance to the player chunk. To get the
   * offsets for a specific render distance, filter
   * by the distance property.
   *
   * @type {{
   *   x: number;
   *   z: number;
   *   distance: number;
   * }[]}
   */
  const offsets = [
    ...flat(
      map(range(-maxRenderDistance, maxRenderDistance), (x) =>
        map(range(-maxRenderDistance, maxRenderDistance), (z) => [x, z])
      )
    ),
  ]
    .sort((a, b) => lengthSquared(a) - lengthSquared(b))
    .map(([x, z]) => ({ x, z, distance: Math.max(Math.abs(x), Math.abs(z)) }));

  /**
   * Offsets grouped by exact render distance
   * @typedef {{
   *   distance: number;
   *   i: number;
   *   offsets: {
   *     x: number;
   *     z: number;
   *   }[];
   * }} OffsetGroup
   * @type {OffsetGroup[]}
   */
  const grouped_offsets = [];

  /**
   * Amount of groups per exact render distance
   * @type {number[]}
   */
  const distance_group_counts = new Array(maxRenderDistance + 1).fill(0);
  {
    /**
     * @type {OffsetGroup}
     */
    let group = { distance: 0, i: 0, offsets: [] };
    distance_group_counts[0]++;
    for (let { x, z, distance } of offsets) {
      if (group.distance == distance) {
        group.offsets.push({ x, z });
      } else {
        grouped_offsets.push(group);
        group = {
          distance,
          i: distance_group_counts[distance]++,
          offsets: [{ x, z }],
        };
      }
    }
    grouped_offsets.push(group);
  }

  /**
   * @param {OffsetGroup} group
   */
  const getVarName = (group) => `OFFSETS_D${group.distance}_${group.i}`;

  file.newLine();

  for (let group of grouped_offsets) {
    file.writeLine(
      `const ${getVarName(group)}: [(i8,i8); ${
        group.offsets.length
      }] = [${group.offsets.map(({ x, z }) => `(${x},${z})`).join(",")}];`
    );
  }

  file.newLine();

  file.writeLine(
    `const OFFSETS: [(u8, &'static [(i8,i8)]); ${grouped_offsets.length}] = [`,
    joinArrayFormatted(
      grouped_offsets.map(
        (offset) => `(${offset.distance}, &${getVarName(offset)})`
      ),
      "  ",
      ", ",
      120
    ),
    "];"
  );

  file.newLine();

  file.writeLine(
    `const GROUPS: [u8; ${
      distance_group_counts.length
    }] = [${distance_group_counts.join(", ")}];`
  );

  file.newLine();

  file.documentLine(
    "Returns a `Vec` containing the positions of all chunks that",
    "have to be loaded, sorted by their distance to the given offset."
  );
  const fn = new lib.FunctionGenerator(
    file,
    "get_chunks_to_load",
    "Vec<ChunkPosition>",
    true
  );
  fn.addParam(
    //
    "render_distance",
    "u8"
  )(
    //
    "offset",
    "ChunkPosition"
  )(
    //
    "loaded",
    "&HashSet<ChunkPosition>"
  );
  fn.writeLine(
    `\
assert!(
  render_distance <= MAX_RENDER_DISTANCE,
  "Given render distance is greater than the supported distance ({} > {})",
  render_distance, MAX_RENDER_DISTANCE
);
let mut chunks: Vec<ChunkPosition> = Vec::new();
// Counts down the highest distance group
// to have a possible short circuit
let mut counter: u8 = GROUPS[render_distance as usize];
for (d, off) in OFFSETS.iter() {
  if *d <= render_distance {
    if *d == render_distance {
      counter -= 1;
    }
    for (x, z) in *off {
      let chunk = ChunkPosition::new(offset.x + (*x as i32), offset.z + (*z as i32));
      if !loaded.contains(&chunk) {
        chunks.push(chunk)
      }
    }
  }
  if counter == 0 {
    break;
  }
}
chunks`
  );

  fn.finish();
}

file.finish();

/**
 * Generates numbers from min to max.
 * @param {number} min Start value
 * @param {number} max End value (inclusive)
 */
function* range(min, max) {
  for (var i = min; i <= max; i++) yield i;
}

/**
 * Maps the output of a generator using a function.
 * @param {Generator<I, void, unknown>} iter Iterator to map
 * @param {(input: I) => O} fn Mapping function
 * @template I, O
 */
function* map(iter, fn) {
  for (var i of iter) {
    yield fn(i);
  }
}

/**
 * Flattens an generator which generates an iterator.
 * @param {Generator<Iterable<T>, void, unknown>} iter Iterator to flatten
 * @template T
 */
function* flat(iter) {
  for (var i of iter) {
    for (var j of i) {
      yield j;
    }
  }
}

/**
 * Calculates the squared length of a vector
 * @param {number[]} vec Vector
 */
function lengthSquared(vec) {
  return vec.reduce((r, x) => r + x * x, 0);
}

/**
 * Rough code to join an array while keeping the
 * text under a width limit for better viewing in
 * an editor.
 * @param {string[]} array Array to join
 */
function joinArrayFormatted(
  array,
  linePrefix = "",
  delimiter = ", ",
  maxWidth = 150
) {
  const { ret, str } = array.reduce(
    ({ str, ret }, val) =>
      str.length + val.length + delimiter.length > maxWidth
        ? { str: linePrefix + val + delimiter, ret: [...ret, str] }
        : { str: str + val + delimiter, ret },
    { str: linePrefix, ret: [] }
  );
  return [...ret, str].join("\n");
}
