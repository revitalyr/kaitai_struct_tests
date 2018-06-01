#include <boost/test/unit_test.hpp>
#include <enum_deep.h>
#include <iostream>
#include <fstream>
#include <vector>

BOOST_AUTO_TEST_CASE(test_enum_deep) {
    std::ifstream ifs("src/enum_0.bin", std::ifstream::binary);
    kaitai::kstream ks(&ifs);
    enum_deep_t* r = new enum_deep_t(&ks);

    BOOST_CHECK_EQUAL(r->pet_1(), enum_deep_t::container1_t::ANIMAL_CAT);
    BOOST_CHECK_EQUAL(r->pet_2(), enum_deep_t::container1_t::container2_t::ANIMAL_HARE);

    delete r;
}