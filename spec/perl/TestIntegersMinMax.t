# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestIntegersMinMax;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use IntegersMinMax;

sub test_integers_min_max: Test(28) {
    my $r = IntegersMinMax->from_file('src/integers_min_max.bin');

    is($r->unsigned_min()->u1(), 0, 'Equals');
    is($r->unsigned_min()->u2le(), 0, 'Equals');
    is($r->unsigned_min()->u4le(), 0, 'Equals');
    is($r->unsigned_min()->u8le(), 0, 'Equals');
    is($r->unsigned_min()->u2be(), 0, 'Equals');
    is($r->unsigned_min()->u4be(), 0, 'Equals');
    is($r->unsigned_min()->u8be(), 0, 'Equals');
    is($r->unsigned_max()->u1(), 255, 'Equals');
    is($r->unsigned_max()->u2le(), 65535, 'Equals');
    is($r->unsigned_max()->u4le(), 4294967295, 'Equals');
    is($r->unsigned_max()->u8le(), 18446744073709551615, 'Equals');
    is($r->unsigned_max()->u2be(), 65535, 'Equals');
    is($r->unsigned_max()->u4be(), 4294967295, 'Equals');
    is($r->unsigned_max()->u8be(), 18446744073709551615, 'Equals');
    is($r->signed_min()->s1(), -128, 'Equals');
    is($r->signed_min()->s2le(), -32768, 'Equals');
    is($r->signed_min()->s4le(), -2147483648, 'Equals');
    is($r->signed_min()->s8le(), -9223372036854775808, 'Equals');
    is($r->signed_min()->s2be(), -32768, 'Equals');
    is($r->signed_min()->s4be(), -2147483648, 'Equals');
    is($r->signed_min()->s8be(), -9223372036854775808, 'Equals');
    is($r->signed_max()->s1(), 127, 'Equals');
    is($r->signed_max()->s2le(), 32767, 'Equals');
    is($r->signed_max()->s4le(), 2147483647, 'Equals');
    is($r->signed_max()->s8le(), 9223372036854775807, 'Equals');
    is($r->signed_max()->s2be(), 32767, 'Equals');
    is($r->signed_max()->s4be(), 2147483647, 'Equals');
    is($r->signed_max()->s8be(), 9223372036854775807, 'Equals');
}

Test::Class->runtests;
