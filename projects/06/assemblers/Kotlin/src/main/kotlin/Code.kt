package main.kotlin

class Code() {
    companion object {
        private val DESTINATION : HashMap<String, String> = hashMapOf(
            "M" to "001",
            "D" to "010",
            "MD" to "011",
            "A" to "100",
            "AM" to "101",
            "AD" to "110",
            "AMD" to "111"
        )

        private val COMPUTATION : HashMap<String, String> = hashMapOf(
            "0" to "0101010",
            "1" to "0111111",
            "-1" to "0111010",
            "D" to "0001100",
            "A" to "0110000",
            "M" to "1110000",
            "!D" to "0001101",
            "!A" to "0110001",
            "!M" to "1110001",
            "-D" to "0001111",
            "-A" to "0110011",
            "-M" to "1110011",
            "D+1" to "0011111",
            "A+1" to "0110111",
            "M+1" to "1110111",
            "D-1" to "0001110",
            "A-1" to "0110010",
            "M-1" to "1110010",
            "D+A" to "0000010",
            "D+M" to "1000010",
            "D-A" to "0010011",
            "D-M" to "1010011",
            "A-D" to "0000111",
            "M-D" to "1000111",
            "D&A" to "0000000",
            "D&M" to "1000000",
            "D|A" to "0010101",
            "D|M" to "1010101"
        )

        private val JUMP : HashMap<String, String> = hashMapOf(
            "JGT" to "001",
            "JEQ" to "010",
            "JGE" to "011",
            "JLT" to "100",
            "JNE" to "101",
            "JLE" to "110",
            "JMP" to "111"
        )
    }

    private fun genDest(mnemonic : String) : String {
        return DESTINATION.getOrElse(mnemonic) { "000" }
    }

    private fun genComp(mnemonic : String) : String? {
        return COMPUTATION[mnemonic]
    }

    private fun genJump(mnemonic : String) : String {
        return JUMP.getOrElse(mnemonic) { "000" }
    }

    fun genCommand(commandComponents: Parser.Command): String {
        val (dest, comp, jump) = commandComponents
        return "111" + genDest(dest) + genComp(comp) + genJump(jump)
    }

}