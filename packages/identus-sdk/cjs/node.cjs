// SPDX-License-Identifier: Apache-2.0
"use strict";

// CommonJS callers retain an asynchronous provider factory while sharing the
// exact ESM facade implementation. No parser or projection logic is duplicated.
module.exports = Object.freeze({
  createDidProvider(options) {
    return import("../dist/node.js").then((module) =>
      module.createDidProvider(options),
    );
  },
});
