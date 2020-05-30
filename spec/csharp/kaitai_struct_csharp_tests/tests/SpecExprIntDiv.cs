// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecExprIntDiv : CommonSpec
    {
        [Test]
        public void TestExprIntDiv()
        {
            var r = ExprIntDiv.FromFile(SourceFile("fixed_struct.bin"));

            Assert.AreEqual(r.IntU, 1262698832);
            Assert.AreEqual(r.IntS, -52947);
            Assert.AreEqual(r.DivPosConst, 756);
            Assert.AreEqual(r.DivNegConst, -757);
            Assert.AreEqual(r.DivPosSeq, 97130679);
            Assert.AreEqual(r.DivNegSeq, -4073);
        }
    }
}