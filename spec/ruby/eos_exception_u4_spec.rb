require 'eos_exception_u4'

RSpec.describe EosExceptionU4 do
  it 'parses test properly' do
    expect {
      r = EosExceptionU4.from_file('src/term_strz.bin')
    }.to raise_error(EOFError)
  end
end
