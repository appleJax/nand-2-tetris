package main.kotlin

class Assembler {
    fun assemble(program: String): String {
        val parser = Parser(program)
        val symTable = SymbolTable()
        val code = Code()

        // First pass to record symbols
        var currentLine = 0
        while (parser.hasMoreCommands()) {
            val commandType: CommandType = parser.commandType()

            if (commandType == CommandType.L_COMMAND) {
                symTable.addEntry(parser.symbol(), currentLine)
            } else {
                currentLine++
            }

            parser.advance()
        }

        parser.reset()
        val assemblyCode = StringBuilder()

        while (parser.hasMoreCommands()) {
            val commandType: CommandType = parser.commandType()

            if (commandType == CommandType.C_COMMAND) {
                assemblyCode.append(code.genCommand(parser.parseCommand()))
                assemblyCode.append("\n")
            } else if (commandType == CommandType.A_COMMAND) {
                val sym = parser.symbol()
                val address = when {
                    (sym.toIntOrNull() != null) -> sym.toInt()
                    (symTable.contains(sym)) -> symTable.GetAddress(sym)
                    else -> {
                        symTable.addEntry(sym)
                        symTable.GetAddress(sym)
                    }
                }
                assemblyCode.append(address?.toString(2)?.padStart(16, '0'))
                assemblyCode.append("\n")
            }

            parser.advance()
        }

        return assemblyCode.toString()
    }
}