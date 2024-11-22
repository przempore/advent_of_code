#include <fmt/core.h>
#include <fmt/ranges.h>

#include <algorithm>
#include <array>
#include <cstdint>
#include <fstream>
#include <numeric>
#include <sstream>
#include <string>
#include <vector>

void part_one() {
    std::ifstream file("input.txt");

    std::vector<uint64_t> sums{};
    uint64_t partial_sum{0};
    std::string line{};
    auto max = partial_sum;
    while (std::getline(file, line)) {
        // fmt::println("line: {}", line);
        if (line.empty() and partial_sum != 0) {
            sums.push_back(partial_sum);
            partial_sum = 0;
        }

        std::istringstream iss(line);
        auto tmp = partial_sum;
        iss >> partial_sum;
        partial_sum += tmp;

        if (max < partial_sum) {
            max = partial_sum;
        }
    }

    sums.push_back(partial_sum);

    fmt::println("part one sum: {}", max);
}

void part_two() {
    std::ifstream file("input.txt");

    std::vector<uint64_t> sums{};
    uint64_t partial_sum{0};
    std::string line{};
    std::array max = {partial_sum, partial_sum, partial_sum};
    while (std::getline(file, line)) {
        if (line.empty() and partial_sum != 0) {
            sums.push_back(partial_sum);
            partial_sum = 0;
        }

        std::istringstream iss(line);
        auto tmp = partial_sum;
        iss >> partial_sum;
        partial_sum += tmp;
    }
    sums.push_back(partial_sum);

    for (auto& s : sums) {
        auto current_min = std::ranges::min_element(max.begin(), max.end());
        if (*current_min < s) {
            *current_min = s;
        }
    }

    // fmt::print("{}", fmt::join(max, ", "));
    // fmt::println("\n+++++++");

    fmt::println("part two sum: {}", std::accumulate(max.begin(), max.end(), 0));
}

int main() {
    part_one();
    part_two();

    return 0;
}
