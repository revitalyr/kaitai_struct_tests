meta:
  id: valid_fail_max_int
seq:
  - id: foo
    type: u1
    valid:
      min: 12 # there is actually 0x50 in the file
