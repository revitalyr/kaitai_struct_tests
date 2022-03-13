local luaunit = require("luaunit")

require("default_endian_expr_exception")

TestDefaultEndianExprException = {}

function TestDefaultEndianExprException:test_default_endian_expr_exception()
    luaunit.assertErrorMsgMatches(".+: unable to decide endianness", DefaultEndianExprException.from_file, DefaultEndianExprException, "src/endian_expr.bin")
end
