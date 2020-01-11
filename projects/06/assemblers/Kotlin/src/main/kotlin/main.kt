package main.kotlin

import java.io.File
import java.nio.file.Paths

fun main(args: Array<String>) {
    var filepath = "src/test/kotlin/testProgram.asm";
    if (args.isNotEmpty()) {
        filepath = args[0]
    }

    val rawProgram: String = File(Paths.get(filepath).toString()).readText(Charsets.UTF_8)
    val parser = Parser(rawProgram)
    val symTable = SymbolTable()
    val code = Code()

    val outputPath = filepath.replace(".asm", ".hack")

    File(Paths.get(outputPath).toString()).bufferedWriter().use { out ->

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
        while (parser.hasMoreCommands()) {
            val commandType: CommandType = parser.commandType()

            if (commandType == CommandType.C_COMMAND) {
                out.write(code.genCommand(parser.parseCommand()))
                out.write("\n")
            } else if (commandType == CommandType.A_COMMAND) {
                val sym = parser.symbol()
                if (symTable.contains(sym)) {
                    out.write(symTable.GetAddress(sym)?.toString(2)?.padStart(16, '0'))
                    out.write("\n")
                } else {
                    symTable.addEntry(sym)
                    out.write(sym.toInt().toString(2).padStart(16, '0'))
                    out.write("\n")
                }
            }

            parser.advance()
        }
    }
}
