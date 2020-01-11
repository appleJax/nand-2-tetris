import org.junit.Test
import main.kotlin.SymbolTable
import kotlin.test.assertEquals

class SymbolTableTest {

    @Test
    fun `test addEntry`() {
        val symTable = SymbolTable()
        val  newSym = "something-new"
        assertEquals(false, symTable.contains(newSym))

        symTable.addEntry(newSym)
        assertEquals(true, symTable.contains(newSym))
    }

    @Test
    fun `test contains`() {
        val symTable = SymbolTable()
        assertEquals( true, symTable.contains("R0"))
        assertEquals( true, symTable.contains("R1"))
        assertEquals( true, symTable.contains("R2"))
        assertEquals( true, symTable.contains("R3"))
        assertEquals( true, symTable.contains("R4"))
        assertEquals( true, symTable.contains("R5"))
        assertEquals( true, symTable.contains("R6"))
        assertEquals( true, symTable.contains("R7"))
        assertEquals( true, symTable.contains("R8"))
        assertEquals( true, symTable.contains("R9"))
        assertEquals( true, symTable.contains("R10"))
        assertEquals( true, symTable.contains("R11"))
        assertEquals( true, symTable.contains("R12"))
        assertEquals( true, symTable.contains("R13"))
        assertEquals( true, symTable.contains("R14"))
        assertEquals( true, symTable.contains("R15"))
        assertEquals( true, symTable.contains("SCREEN"))
        assertEquals( true, symTable.contains("KBD"))
        assertEquals( true, symTable.contains("SP"))
        assertEquals( true, symTable.contains("LCL"))
        assertEquals( true, symTable.contains("ARG"))
        assertEquals( true, symTable.contains("THIS"))
        assertEquals( true, symTable.contains("THAT"))
        assertEquals( false, symTable.contains("NOTASYM"))
    }

    @Test
    fun `test GetAddress`() {
        val symTable = SymbolTable()
        assertEquals(0, symTable.GetAddress("R0"))
        assertEquals( 1, symTable.GetAddress("R1"))
        assertEquals( 2, symTable.GetAddress("R2"))
        assertEquals( 3, symTable.GetAddress("R3"))
        assertEquals( 4, symTable.GetAddress("R4"))
        assertEquals( 5, symTable.GetAddress("R5"))
        assertEquals( 6, symTable.GetAddress("R6"))
        assertEquals( 7, symTable.GetAddress("R7"))
        assertEquals( 8, symTable.GetAddress("R8"))
        assertEquals( 9, symTable.GetAddress("R9"))
        assertEquals( 10, symTable.GetAddress("R10"))
        assertEquals( 11, symTable.GetAddress("R11"))
        assertEquals( 12, symTable.GetAddress("R12"))
        assertEquals( 13, symTable.GetAddress("R13"))
        assertEquals( 14, symTable.GetAddress("R14"))
        assertEquals( 15, symTable.GetAddress("R15"))
        assertEquals( 16384, symTable.GetAddress("SCREEN"))
        assertEquals( 24576, symTable.GetAddress("KBD"))
        assertEquals( 0, symTable.GetAddress("SP"))
        assertEquals( 1, symTable.GetAddress("LCL"))
        assertEquals( 2, symTable.GetAddress("ARG"))
        assertEquals( 3, symTable.GetAddress("THIS"))
        assertEquals( 4, symTable.GetAddress("THAT"))

        symTable.addEntry("newSymbol1")
        assertEquals( 16, symTable.GetAddress("newSymbol1"))
        symTable.addEntry("newSymbol2")
        assertEquals( 17, symTable.GetAddress("newSymbol2"))
    }
}