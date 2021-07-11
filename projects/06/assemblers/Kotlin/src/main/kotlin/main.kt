package main.kotlin

import java.io.File
import java.nio.file.Paths

fun main(args: Array<String>) {
    var filepath = "src/test/kotlin/testProgram.asm";
    if (args.isNotEmpty()) {
        filepath = args[0]
    }

    val rawProgram: String = File(Paths.get(filepath).toString()).readText(Charsets.UTF_8)
    val assembler = Assembler()
    val assemblyCode = assembler.assemble(rawProgram)

    val outputPath = filepath.replace(".asm", ".hack")
    File(Paths.get(outputPath).toString()).writeText(assemblyCode)
}
