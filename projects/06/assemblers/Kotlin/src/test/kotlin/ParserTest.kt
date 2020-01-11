import main.kotlin.CommandType
import main.kotlin.Parser

import org.junit.Test
import kotlin.test.assertEquals

class ParserTest {

    @Test
    fun `advance`() {
        val rawProgram = """
            one
            two
            three
        """

        val parser = Parser(rawProgram)
        assertEquals(0, parser.getCurrentLine())

        parser.advance()
        assertEquals(1, parser.getCurrentLine())

        parser.advance()
        assertEquals(2, parser.getCurrentLine())

        parser.advance()
        assertEquals(3, parser.getCurrentLine())

        parser.advance()
        assertEquals(3, parser.getCurrentLine())
    }

    @Test
    fun `reset`() {
        val rawProgram = """
            one
            two
            three
        """

        val parser = Parser(rawProgram)
        assertEquals(0, parser.getCurrentLine())

        parser.advance()
        parser.advance()
        assertEquals(2, parser.getCurrentLine())

        parser.reset()
        assertEquals(0, parser.getCurrentLine())
    }

    @Test
    fun `hasMoreCommands`() {
        val rawProgram = """
            // Sample assembly program
            // with comments
            
            @10
            A=D
        """

        val parser = Parser(rawProgram)
        assertEquals(true, parser.hasMoreCommands())

        parser.advance()
        assertEquals(true, parser.hasMoreCommands())

        parser.advance()
        assertEquals(false, parser.hasMoreCommands())
    }

    @Test
    fun `hasMoreCommands - empty program`() {
        val rawProgram = """
            // Sample assembly program
            // with comments
            
            // No commands after comments/whitespace are removed
        """

        val parser = Parser(rawProgram)
        assertEquals( false, parser.hasMoreCommands())
    }

    @Test
    fun `commandType`() {
        val rawProgram = """
            @10
            (LABEL)
            A=D
        """

        val parser = Parser(rawProgram)
        assertEquals(CommandType.A_COMMAND, parser.commandType())

        parser.advance()
        assertEquals(CommandType.L_COMMAND, parser.commandType())

        parser.advance()
        assertEquals(CommandType.C_COMMAND, parser.commandType())
    }

    @Test
    fun `symbol`() {
        val rawProgram = """
            (ONE)
            (TWO)
            @12
            @SYMBOL
        """

        val parser = Parser(rawProgram)
        assertEquals("ONE", parser.symbol())

        parser.advance()
        assertEquals("TWO", parser.symbol())

        parser.advance()
        assertEquals("12", parser.symbol())

        parser.advance()
        assertEquals("SYMBOL", parser.symbol())
    }

    @Test
    fun `symbol - invalid call on C_COMMAND`() {
        val rawProgram = """
            C_COMMAND
        """

        val parser = Parser(rawProgram)
        assertEquals("", parser.symbol())
    }

    @Test
    fun `parseCommand`() {
        val rawProgram = """
            AMD=!D;JEQ
            A=M
            0;JMP
        """

        val command1 = Parser.Command("AMD", "!D", "JEQ")
        val command2 = Parser.Command("A", "M", "")
        val command3 = Parser.Command("", "0", "JMP")

        val parser = Parser(rawProgram)
        assertEquals(command1, parser.parseCommand())

        parser.advance()
        assertEquals(command2, parser.parseCommand())

        parser.advance()
        assertEquals(command3, parser.parseCommand())
    }
}
