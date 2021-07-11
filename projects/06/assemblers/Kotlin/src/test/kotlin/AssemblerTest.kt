import org.junit.Test
import kotlin.test.assertEquals
import main.kotlin.Assembler

class AssemblerTest {
    @Test
    fun `test assemble`() {
        val program = """
           // Test Program
           
           @42
           D=M;JLT // will never jump
           
           @foo // not used
           @bar
           M=D
           
           (END)
           @END
           0;JMP
           
        """

        val assembler = Assembler()
        val output = listOf(
            "0000000000101010",
            "1111110000010100",
            "0000000000010000",
            "0000000000010001",
            "1110001100001000",
            "0000000000000101",
            "1110101010000111\n"
        ).joinToString("\n")

        assertEquals(output, assembler.assemble(program))
    }
}