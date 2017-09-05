local luaunit = require("luaunit")

require("nested_types")

TestNestedTypes = {}

function TestNestedTypes:test_nested_types()
    local r = NestedTypes:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.one.typed_at_root.value_b, 80)
    luaunit.assertEquals(r.one.typed_here.value_c, 65)
    luaunit.assertEquals(r.two.value_b, 67)
end