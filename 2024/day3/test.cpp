#include "solution.hpp"

#include <gtest/gtest.h>

using namespace ::testing;

TEST(solution_test, extract_numbers) {
    auto numbers = extract_numbers("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    std::vector expected = {2, 4, 5, 5, 11, 8, 8, 5};
    EXPECT_EQ(numbers, expected);
}

TEST(solution_test, part_1) {
    EXPECT_EQ(161, part_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"));
}
