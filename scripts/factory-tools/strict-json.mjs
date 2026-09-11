#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

function fail(message) {
  throw new Error(message);
}

export function parseJsonWithoutDuplicates(text, { maximumDepth = 64, maximumNodes = 100_000 } = {}) {
  if (typeof text !== "string") fail("JSON input must be text");
  let offset = 0;
  let nodes = 0;

  function skipWhitespace() {
    while (offset < text.length && /[\u0009\u000a\u000d\u0020]/u.test(text[offset])) offset += 1;
  }

  function countNode(depth) {
    nodes += 1;
    if (nodes > maximumNodes) fail("JSON input exceeds the node bound");
    if (depth > maximumDepth) fail("JSON input exceeds the nesting bound");
  }

  function parseString() {
    const start = offset;
    if (text[offset] !== '"') fail("JSON string expected");
    offset += 1;
    while (offset < text.length) {
      const character = text[offset];
      if (character === '"') {
        offset += 1;
        try {
          return JSON.parse(text.slice(start, offset));
        } catch {
          fail("JSON string is malformed");
        }
      }
      if (character === "\\") {
        offset += 1;
        if (offset >= text.length) fail("JSON string is malformed");
        if (text[offset] === "u") {
          const escape = text.slice(offset + 1, offset + 5);
          if (!/^[0-9a-fA-F]{4}$/u.test(escape)) fail("JSON string is malformed");
          offset += 5;
        } else {
          if (!/["\\/bfnrt]/u.test(text[offset])) fail("JSON string is malformed");
          offset += 1;
        }
        continue;
      }
      if (character.charCodeAt(0) <= 0x1f) fail("JSON string is malformed");
      offset += 1;
    }
    fail("JSON string is unterminated");
  }

  function parseNumber() {
    const match = /^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/u.exec(text.slice(offset));
    if (!match) fail("JSON number is malformed");
    offset += match[0].length;
    const value = Number(match[0]);
    if (!Number.isFinite(value)) fail("JSON number is outside the finite range");
    return value;
  }

  function parseArray(depth) {
    const values = [];
    offset += 1;
    skipWhitespace();
    if (text[offset] === "]") {
      offset += 1;
      return values;
    }
    while (offset < text.length) {
      values.push(parseValue(depth + 1));
      skipWhitespace();
      if (text[offset] === "]") {
        offset += 1;
        return values;
      }
      if (text[offset] !== ",") fail("JSON array separator expected");
      offset += 1;
      skipWhitespace();
    }
    fail("JSON array is unterminated");
  }

  function parseObject(depth) {
    const value = Object.create(null);
    const keys = new Set();
    offset += 1;
    skipWhitespace();
    if (text[offset] === "}") {
      offset += 1;
      return value;
    }
    while (offset < text.length) {
      const key = parseString();
      if (keys.has(key)) fail("JSON object contains a duplicate field");
      keys.add(key);
      skipWhitespace();
      if (text[offset] !== ":") fail("JSON object separator expected");
      offset += 1;
      value[key] = parseValue(depth + 1);
      skipWhitespace();
      if (text[offset] === "}") {
        offset += 1;
        return value;
      }
      if (text[offset] !== ",") fail("JSON object member separator expected");
      offset += 1;
      skipWhitespace();
    }
    fail("JSON object is unterminated");
  }

  function parseValue(depth) {
    skipWhitespace();
    countNode(depth);
    const character = text[offset];
    if (character === '"') return parseString();
    if (character === "{") return parseObject(depth);
    if (character === "[") return parseArray(depth);
    if (text.startsWith("true", offset)) {
      offset += 4;
      return true;
    }
    if (text.startsWith("false", offset)) {
      offset += 5;
      return false;
    }
    if (text.startsWith("null", offset)) {
      offset += 4;
      return null;
    }
    if (character === "-" || /[0-9]/u.test(character ?? "")) return parseNumber();
    fail("JSON value is malformed");
  }

  const value = parseValue(0);
  skipWhitespace();
  if (offset !== text.length) fail("JSON input has trailing data");
  return value;
}
