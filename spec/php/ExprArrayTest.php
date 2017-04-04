<?php
namespace Kaitai\Struct\Tests;

class ExprArrayTest extends TestCase {
    public function testExprArray() {
        $r = ExprArray::fromFile(self::SRC_DIR_PATH . "/expr_array.bin");

        $this->assertEquals(4, $r->aintSize);
        $this->assertEquals(7657765, $r->aintFirst);
        $this->assertEquals(16272640, $r->aintLast);
        $this->assertEquals(49185, $r->aintMin);
        $this->assertEquals(1123362332, $r->aintMax);

        $this->assertEquals(3, $r->afloatSize);
        $this->assertEquals(-2.6839530254859364e-121, $r->afloatFirst);
        $this->assertEquals(-1.1103359815095273e-175, $r->afloatLast);
        $this->assertEquals(-8.754689149998834e+288, $r->afloatMin);
        $this->assertEquals(-1.1103359815095273e-175, $r->afloatMax);

        $this->assertEquals(3, $r->astrSize);
        $this->assertEquals('foo', $r->astrFirst);
        $this->assertEquals('baz', $r->astrLast);
        $this->assertEquals('bar', $r->astrMin);
        $this->assertEquals('foo', $r->astrMax);
    }
}
