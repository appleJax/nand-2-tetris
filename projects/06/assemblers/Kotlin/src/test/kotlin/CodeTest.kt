import org.junit.Test
import main.kotlin.Code
import main.kotlin.Parser
import kotlin.test.assertEquals

class CodeTest {

    @Test
    fun `genCommand`() {
        val code = Code()
        val assemblyCommand1 = Parser.Command("M", "D+1", "JEQ")
        assertEquals("1110010011111010", code.genCommand(assemblyCommand1))

        val assemblyCommand2 = Parser.Command("AD", "D&M", "JLT")
        assertEquals("1111101000000100", code.genCommand(assemblyCommand2))
    }
}