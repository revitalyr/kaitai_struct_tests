<?php
// Autogenerated from KST: please remove this line if doing any edits by hand!

namespace Kaitai\Struct\Tests;

class EnumForUnknownIdTest extends TestCase {
    public function testEnumForUnknownId() {
        $r = EnumForUnknownId::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');

        $this->assertSame(80, $r->one());
    }
}
