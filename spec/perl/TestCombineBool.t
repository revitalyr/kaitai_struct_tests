# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestCombineBool;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use CombineBool;

sub test_combine_bool: Test(2) {
    my $r = CombineBool->from_file('src/enum_negative.bin');

    cmp_ok($r->bool_bit(), '==', 1, 'Equals');
    cmp_ok($r->bool_calc_bit(), '==', 0, 'Equals');
}

Test::Class->runtests;
