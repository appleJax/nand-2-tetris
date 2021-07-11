const fs = require("fs");
const path = require("path");

const COMP = {
  0: "0101010",
  1: "0111111",
  "-1": "0111010",
  D: "0001100",
  A: "0110000",
  M: "1110000",
  "!D": "0001101",
  "!A": "0110001",
  "!M": "1110001",
  "-D": "0001111",
  "-A": "0110011",
  "-M": "1110011",
  "D+1": "0011111",
  "A+1": "0110111",
  "M+1": "1110111",
  "D-1": "0001110",
  "A-1": "0110010",
  "M-1": "1110010",
  "D+A": "0000010",
  "D+M": "1000010",
  "D-A": "0010011",
  "D-M": "1010011",
  "A-D": "0000111",
  "M-D": "1000111",
  "D&A": "0000000",
  "D&M": "1000000",
  "D|A": "0010101",
  "D|M": "1010101",
};

const DEST = {
  null: "000",
  M: "001",
  D: "010",
  MD: "011",
  A: "100",
  AM: "101",
  AD: "110",
  AMD: "111",
};

const JUMP = {
  null: "000",
  JGT: "001",
  JEQ: "010",
  JGE: "011",
  JLT: "100",
  JNE: "101",
  JLE: "110",
  JMP: "111",
};

const SYMBOLS = {
  R0: 0,
  R1: 1,
  R2: 2,
  R3: 3,
  R4: 4,
  R5: 5,
  R6: 6,
  R7: 7,
  R8: 8,
  R9: 9,
  R10: 10,
  R11: 11,
  R12: 12,
  R13: 13,
  R14: 14,
  R15: 15,
  SCREEN: 16384,
  KBD: 24576,
  SP: 0,
  LCL: 1,
  ARG: 2,
  THIS: 3,
  THAT: 4,
};

const defaultCommandMap = {
  COMP_TABLE: COMP,
  DEST_TABLE: DEST,
  JUMP_TABLE: JUMP,
};

let SYMBOLIC_ADDRESS = {
  value: 16,
};

if (require.main === module) {
  // This file was executed from the command line as a script
  const [, , rawFilepath] = process.argv;
  const filepath = path.resolve(rawFilepath);

  if (!fs.existsSync(filepath)) {
    console.log(
      "File does not exist. Please double check the filepath:",
      filepath
    );
    process.exit();
  }

  const hackFilepath = filepath.replace(/\.[^.]*$/, ".hack");
  const writeStream = fs.createWriteStream(hackFilepath);

  writeStream.on("finish", () =>
    console.log("Assembly complete:", hackFilepath)
  );

  const program = fs.readFileSync(filepath, "utf8");

  const normalizedLines = cleanProgramAndRecordJumpSymbols(program);

  // Translate to machine code
  normalizedLines.forEach((line) => writeStream.write(`${translate(line)}\n`));

  writeStream.end();
}

function cleanProgramAndRecordJumpSymbols(rawProgram, SYM_TABLE = SYMBOLS) {
  let currentLine = 0;
  let line;

  return rawProgram.split("\n").reduce((result, rawLine) => {
    line = normalize(rawLine);
    if (!line) {
      return result;
    }

    // Jump Symbols
    if (line.startsWith("(")) {
      SYM_TABLE[line.slice(1, -1)] = currentLine;
      return result;
    }

    currentLine++;
    return result.concat(line);
  }, []);
}

function normalize(line) {
  return line.replace(/\/\/.*/, "").replace(/\s/g, "");
}

function translate(line) {
  return line.startsWith("@") ? translateA(line.slice(1)) : translateC(line);
}

function translateA(
  identifier,
  SYM_TABLE = SYMBOLS,
  symbolicAddress = SYMBOLIC_ADDRESS
) {
  const address = handleSymbol(identifier, SYM_TABLE, symbolicAddress);
  return address.toString(2).padStart(16, "0");
}

function handleSymbol(
  identifier,
  SYM_TABLE = SYMBOLS,
  symbolicAddress = SYMBOLIC_ADDRESS
) {
  let address = Number(identifier);
  if (isNaN(address)) {
    address = SYM_TABLE[identifier];
    if (address === undefined) {
      address = symbolicAddress.value;
      SYM_TABLE[identifier] = symbolicAddress.value;
      symbolicAddress.value++;
    }
  }
  return address;
}

function translateC(line, commandMap = defaultCommandMap) {
  const instructionParts = parseCInstruction(line);
  return formatCBinary(instructionParts, commandMap);
}

function parseCInstruction(instruction) {
  let [destination, rest] = instruction.split("=");
  if (!rest) {
    rest = destination;
    destination = null;
  }

  const [computation, jump = null] = rest.split(";");

  return {
    computation,
    destination,
    jump,
  };
}

function formatCBinary(
  { computation, destination, jump },
  { COMP_TABLE, DEST_TABLE, JUMP_TABLE } = defaultCommandMap
) {
  return `111${COMP_TABLE[computation]}${DEST_TABLE[destination]}${JUMP_TABLE[jump]}`;
}

module.exports = {
  cleanProgramAndRecordJumpSymbols,
  formatCBinary,
  handleSymbol,
  normalize,
  parseCInstruction,
  translate,
  translateA,
  translateC,
};
