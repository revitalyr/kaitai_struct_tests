package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.ClassTypeProvider
import _root_.io.kaitai.struct.datatype.DataType
import _root_.io.kaitai.struct.datatype.{
  KSError,
  ValidationNotEqualError,
  UndecidedEndiannessError,
  EndOfStreamError
}
import _root_.io.kaitai.struct.datatype.DataType._
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.LuaCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import _root_.io.kaitai.struct.translators.LuaTranslator

class LuaSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  importList.add("local luaunit = require(\"luaunit\")")

  val translator = new LuaTranslator(provider, importList)
  val className = LuaCompiler.type2class(spec.id)

  override def fileName(name: String): String = s"test_$name.lua"

  override def indentStr: String = "    "

  override def header(): Unit = {
    out.puts
    out.puts(s"""require("${spec.id}")""")
    out.puts
    out.puts(s"Test$className = {}")
    out.puts
    out.puts(s"function Test$className:test_${spec.id}()")
    out.inc
  }

  override def runParse(): Unit = {
    out.puts(s"""local r = $className:from_file("src/${spec.data}")""")
  }

  override def runParseExpectError(exception: KSError): Unit = {
    val msg = exception match {
      case _: ValidationNotEqualError => "not equal, expected .*, but got .*"
      case UndecidedEndiannessError => "unable to decide endianness"
      case EndOfStreamError => "requested %d+ bytes, but only %d+ bytes available"
      case _ => LuaCompiler.ksErrorName(exception)
    }
    out.puts(s"""luaunit.assertErrorMsgMatches(".+: $msg", $className.from_file, $className, "src/${spec.data}")""")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("end")
  }

  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translateExp(check.expected)
    out.puts(s"luaunit.assertEquals($actStr, $expStr)")
  }

  override def floatAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"luaunit.assertAlmostEquals($actStr, $expStr, 0.000001)")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"luaunit.assertNil($actStr)")
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleAssert(check)

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace("self." + Main.INIT_OBJ_NAME, "r")

  def translateExp(x: Ast.expr) =
    translator.translate(x).replace("self._root", className)

  override def results: String =
    "-- " + AUTOGEN_COMMENT + "\n\n" + super.results
}
