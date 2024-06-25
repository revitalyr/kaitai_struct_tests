// Autogenerated from KST: please remove this line if doing any edits by hand!

package spec

import (
	"runtime/debug"
	"os"
	"testing"
	"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai"
	. "test_formats"
	"github.com/stretchr/testify/assert"
)

func TestParamsPassArrayInt(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/position_to_end.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r ParamsPassArrayInt
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	assert.EqualValues(t, 3, len(r.PassInts.Nums))
	assert.EqualValues(t, 513, r.PassInts.Nums[0])
	assert.EqualValues(t, 1027, r.PassInts.Nums[1])
	assert.EqualValues(t, 1541, r.PassInts.Nums[2])
	assert.EqualValues(t, 2, len(r.PassIntsCalc.Nums))
	assert.EqualValues(t, 27643, r.PassIntsCalc.Nums[0])
	assert.EqualValues(t, 7, r.PassIntsCalc.Nums[1])
}