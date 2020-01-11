const {
  cleanProgramAndRecordJumpSymbols,
  formatCBinary,
  handleSymbol,
  normalize,
  parseCInstruction,
  translate,
  translateA,
  translateC
} = require('./assembler');

describe('normalize', () => {
  it('should remove whitespace and comments', () => {
    const result = normalize("  Only Chars   Not\tSpaces // Or comments");
    expect(result).toEqual("OnlyCharsNotSpaces");
  });
});

describe('handleSymbol', () => {
  it('should return numbers as-is', () => {
    const address = 42;
    const result = handleSymbol(address);
    expect(result).toEqual(address);
  });

  it('should return the value of the symbol if it exists', () => {
    const symbol = 'someSymbol';
    const SYMBOLS = { [symbol]: 42 };
    const result = handleSymbol(symbol, SYMBOLS);
    expect(result).toEqual(42);
  });

  it('should record a new symbol entry if one does not currently exist', () => {
    const symbol = 'someSymbol';
    const SYMBOLS = {};
    const result = handleSymbol(symbol, SYMBOLS);

    expect(result).toEqual(SYMBOLS[symbol]);
    expect(SYMBOLS[symbol]).not.toBeUndefined();
  });

  it('should accept a nextAddress parameter for storing a new symbol', () => {
    const symbol = 'someSymbol';
    const SYMBOLS = {};
    const nextAddress = 37;
    const result = handleSymbol(symbol, SYMBOLS, 37);

    expect(result).toEqual(SYMBOLS[symbol]);
    expect(SYMBOLS[symbol]).toEqual(nextAddress);
  });
});

describe('parseCInstruction', () => {
  it('should return the jump, destination, and computation parts of the command', () => {
    const dest = 'destination';
    const comp = 'computation';
    const jmp = 'jump';
    const cInstruction = `${dest}=${comp};${jmp}`;

    const result = parseCInstruction(cInstruction);
    expect(result).toEqual({
      destination: dest,
      computation: comp,
      jump: jmp
    });
  });

  it('should handle instructions with no destination', () => {
    const comp = 'computation';
    const jmp = 'jump';
    const cInstruction = `${comp};${jmp}`;

    const result = parseCInstruction(cInstruction);
    expect(result).toEqual({
      destination: null,
      computation: comp,
      jump: jmp
    })
  });

  it('should handle instructions with no jump', () => {
    const dest = 'destination';
    const comp = 'computation';
    const cInstruction = `${dest}=${comp}`;

    const result = parseCInstruction(cInstruction);
    expect(result).toEqual({
      destination: dest,
      computation: comp,
      jump: null
    });
  });

  it('should handle instructions with no destination or jump', () => {
    const cInstruction = 'computation only';

    const result = parseCInstruction(cInstruction);
    expect(result).toEqual({
      destination: null,
      computation: cInstruction,
      jump: null
    })
  });
});

describe('formatCBinary', () => {
  it('should format a C-instruction correctly with the default Hack commands', () => {
    expect(formatCBinary({
      computation: 'D+M',
      destination: 'A',
      jump: 'JGT'
    })).toEqual('1111000010100001');

    expect(formatCBinary({
      computation: '1',
      destination: 'AMD',
      jump: 'JEQ'
    })).toEqual('1110111111111010');
  });

  it('should accept a custom command mapping', () => {
    const computation = 'exampleComputation';
    const destination = 'exampleDestination';
    const jump = 'exampleJump';

    const COMP_TABLE = { [computation]: 'ccc'};
    const DEST_TABLE = { [destination]: 'ddd'};
    const JUMP_TABLE = { [jump]: 'jjj'};

    const result = formatCBinary(
      {
        computation,
        destination,
        jump
      },
      {
        COMP_TABLE,
        DEST_TABLE,
        JUMP_TABLE
      }
    );

    expect(result).toEqual(`111cccdddjjj`)
  });
});

describe('cleanProgramAndRecordJumpSymbols', () => {
  it('should remove whitespace and comments', () => {
    const rawProgram= `// Some comment
    
      line1

      // another comment

      line2   // inline comment

    `;

    const cleanedProgram = cleanProgramAndRecordJumpSymbols(rawProgram);
    expect(cleanedProgram).toEqual([
      'line1',
      'line2'
    ]);
  });

  it('should record JUMP symbols with the correct line numbers', () => {
    const SYMBOLS = {};
    const jumpSymbol = 'LOOP';

    const rawProgram = `line0
    line1
    (${jumpSymbol})
    line2
    `;
    
    const cleanedProgram = cleanProgramAndRecordJumpSymbols(rawProgram, SYMBOLS);
    expect(cleanedProgram).toEqual([
      'line0',
      'line1',
      'line2'
    ]);
    expect(SYMBOLS[jumpSymbol]).toEqual(2);
  });
});

describe('translateA', () => {
  it('should handle raw addresses', () => {
    expect(translateA(0)).toEqual('0000000000000000')
    expect(translateA(2)).toEqual('0000000000000010')
    expect(translateA(4)).toEqual('0000000000000100')
    expect(translateA(10)).toEqual('0000000000001010')
    expect(translateA(15)).toEqual('0000000000001111')
    expect(translateA(20940)).toEqual('0101000111001100')
  });

  it('should handle symbols', () => {
    const SYMBOLS = {
      exampleSymbol: 5
    };

    expect(translateA('exampleSymbol', SYMBOLS)).toEqual('0000000000000101');
  });

  it('should record new symbols at the next free address', () => {
    const SYMBOLS = {};
    const nextFreeAddress = 5;

    expect(translateA('newSymbol', SYMBOLS, nextFreeAddress))
      .toEqual('0000000000000101');
  });
});

describe('translateC', () => {
  it('should translate a C-instruction to the correct Hack machine code', () => {
    expect(translateC('D=D+M')).toEqual('1111000010010000');
    expect(translateC('0;JMP')).toEqual('1110101010000111');
    expect(translateC('A=A-1;JLE')).toEqual('1110110010100110');
  });

  it('should accept a custom command mapping', () => {
    const customCommands = {
      COMP_TABLE: { comp: 'CCC' },
      DEST_TABLE: { dest: 'DDD' },
      JUMP_TABLE: { jump: 'JJJ' }
    };

    expect(translateC('dest=comp;jump', customCommands)).toEqual('111CCCDDDJJJ');
  });
});

describe('translate', () => {
  it('should translate a real A-instruction', () => {
    expect(translate('@42')).toEqual('0000000000101010');  
    // symbol addresses start at 16
    expect(translate('@symbol')).toEqual('0000000000010000');  
  });

  it('should translate a real C-instruction', () => {
    expect(translate('D=D+M')).toEqual('1111000010010000');
    expect(translate('0;JMP')).toEqual('1110101010000111');
    expect(translate('A=A-1;JLE')).toEqual('1110110010100110');
  });
});
