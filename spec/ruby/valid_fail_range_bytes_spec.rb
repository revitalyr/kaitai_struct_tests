# Autogenerated from KST: please remove this line if doing any edits by hand!

require 'valid_fail_range_bytes'

RSpec.describe ValidFailRangeBytes do
  it 'parses test properly' do
    expect {
      r = ValidFailRangeBytes.from_file('src/fixed_struct.bin')
    }.to raise_error(Kaitai::Struct::ValidationGreaterThanError)
  end
end
