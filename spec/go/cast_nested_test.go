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

func TestCastNested(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			debug.PrintStack()
			t.Fatal("unexpected panic:", r)
		}
	}()
	f, err := os.Open("../../src/switch_opcodes.bin")
	if err != nil {
		t.Fatal(err)
	}
	s := kaitai.NewStream(f)
	var r CastNested
	err = r.Read(s, &r, &r)
	if err != nil {
		t.Fatal(err)
	}

	tmp1, err := r.Opcodes0Str()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "foobar", tmp1.Value)
	tmp2, err := r.Opcodes0StrValue()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, "foobar", tmp2)
	tmp3, err := r.Opcodes1Int()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 66, tmp3.Value)
	tmp4, err := r.Opcodes1IntValue()
	if err != nil {
		t.Fatal(err)
	}
	assert.EqualValues(t, 66, tmp4)
}
