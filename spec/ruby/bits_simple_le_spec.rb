# Autogenerated from KST: please remove this line if doing any edits by hand!

require 'bits_simple_le'

RSpec.describe BitsSimpleLe do
  it 'parses test properly' do
    r = BitsSimpleLe.from_file('src/fixed_struct.bin')

    expect(r.byte_1).to eq 80
    expect(r.byte_2).to eq 65
    expect(r.bits_a).to eq true
    expect(r.bits_b).to eq 1
    expect(r.bits_c).to eq 4
    expect(r.large_bits_1).to eq 331
    expect(r.spacer).to eq 3
    expect(r.large_bits_2).to eq 393
    expect(r.normal_s2).to eq -1
    expect(r.byte_8_9_10).to eq 4407632
    expect(r.byte_11_to_14).to eq 760556875
    expect(r.byte_15_to_19).to eq 1099499455812
    expect(r.byte_20_to_27).to eq 18446744073709551615
    expect(r.test_if_b1).to eq 123
  end
end