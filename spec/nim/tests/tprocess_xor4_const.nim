# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest, os, streams, options, sequtils, ../testhelpers
import ../../../compiled/nim/process_xor4_const

var
  stream = newFileStream("test_out/nim/junitreports/process_xor4_const.xml", fmWrite)
  outputFormatter = newJUnitOutputFormatter(stream)
addOutputFormatter(outputFormatter)

suite "Kaitai Struct Compiler Test Suite":
  test "Nim: ProcessXor4Const":
    let r = ProcessXor4Const.fromFile("src/process_xor_4.bin")

    check(r.key == seq[byte](@[-20, -69, -93, 20].mapIt(toByte(it))))
    check(r.buf == seq[byte](@[102, 111, 111, 32, 98, 97, 114].mapIt(toByte(it))))

close(outputFormatter)