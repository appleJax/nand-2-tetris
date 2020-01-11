package main.kotlin

enum class CommandType { A_COMMAND, C_COMMAND, L_COMMAND }

class Parser(rawProgram: String) {
    private val program = mutableListOf<String>()
    private var currentLine = 0

    data class Command(val dest: String, val comp: String, val jump: String)

    init {
        rawProgram.split(Regex("\r?\n")).forEach {
            val normalizedLine = it.replace(Regex("//.*"), "").trim()
                if (normalizedLine.isNotEmpty()) {
                    program.add(normalizedLine)
                }
        }
    }

    fun getProgram(): List<String> {
        println("Program:")
        println(program.joinToString("\n"))
        return program
    }

    fun hasMoreCommands(): Boolean =
        currentLine < program.size

    fun getCurrentLine(): Int = currentLine

    fun reset() {
        currentLine = 0
    }

    fun advance() {
        if (currentLine < program.size) {
            currentLine++
        }
    }

    fun commandType(): CommandType {
        val firstChar = program[currentLine][0]

        if (firstChar == '@') {
            return CommandType.A_COMMAND
        }

        if (firstChar == '(') {
            return CommandType.L_COMMAND
        }

        return CommandType.C_COMMAND
    }

    fun symbol(): String {
        val command = program[currentLine]
        if (command.startsWith('(')) {
            return command.substring(1, command.length - 1)
        }
        if (command.startsWith('@')) {
            return command.substring(1)
        }
        return ""
    }

    fun parseCommand(): Command {
        val rawCommand = program[currentLine]
        var dest = ""
        var jump = ""

        var compIndexStart = 0;
        var compIndexEnd = rawCommand.length

        if ("=" in rawCommand) {
            val equalsIndex = rawCommand.indexOf('=')
            dest = rawCommand.substring(0, equalsIndex)
            compIndexStart = equalsIndex + 1
        }

        if (";" in rawCommand) {
            val semicolonIndex = rawCommand.indexOf(';')
            jump = rawCommand.substring(semicolonIndex + 1, rawCommand.length)
            compIndexEnd = semicolonIndex
        }

        val comp = rawCommand.substring(compIndexStart, compIndexEnd)

        return Command(dest, comp, jump)
    }
}