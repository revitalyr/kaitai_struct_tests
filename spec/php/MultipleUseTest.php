<?php
// Autogenerated from KST: please remove this line if doing any edits by hand!

namespace Kaitai\Struct\Tests;

class MultipleUseTest extends TestCase {
    public function testMultipleUse() {
        $r = MultipleUse::fromFile(self::SRC_DIR_PATH . '/position_abs.bin');

        $this->assertEquals(32, $r->t1()->firstUse()->value());
        $this->assertEquals(32, $r->t2()->secondUse()->value());
    }
}
