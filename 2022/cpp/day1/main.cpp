#include <fmt/core.h>

#include <fstream>
#include <sstream>
#include <string>
#include <vector>

void part_one() {
    std::ifstream file("input.txt");

    std::vector<uint64_t> sums{};
    uint64_t partial_sum{0};
    std::string line{};
    auto max = partial_sum;
    while (std::getline(file, line)){
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

    // for (const auto& s : sums) {
    //     fmt::println("s: {}", s);
    // }

    fmt::println("max: {}", max);
}

int main() {
    part_one();

    return 0;
}
