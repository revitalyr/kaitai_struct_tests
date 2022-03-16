# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestProcessRepeatUsertype;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ProcessRepeatUsertype;

sub test_process_repeat_usertype: Test(4) {
    my $r = ProcessRepeatUsertype->from_file('src/process_xor_4.bin');

    is(@{$r->blocks()}[0]->a(), -1975704206, 'Equals');
    is(@{$r->blocks()}[0]->b(), 20, 'Equals');
    is(@{$r->blocks()}[1]->a(), 279597642, 'Equals');
    is(@{$r->blocks()}[1]->b(), 68, 'Equals');
}

Test::Class->runtests;
