#include "solution.hpp"

#include <fmt/core.h>
#include <fmt/ranges.h>

#include <iterator>
#include <numeric>
#include <regex>

std::vector<int> extract_numbers(std::string const& file_content) {
    std::regex mul_regex(R"(mul\((\d+),\s*(\d+)\))");
    std::smatch match;
    std::vector<int> numbers;

    auto search_start{file_content.cbegin()};
    while (std::regex_search(search_start, file_content.cend(), match,
                             mul_regex)) {
        numbers.push_back(std::stoi(match[1].str()));
        numbers.push_back(std::stoi(match[2].str()));

        search_start = match.suffix().first;
    }

    return numbers;
}

int part_1(std::string const& file_content) {
    const auto numbers = extract_numbers(file_content);

    std::vector<int> products;
    products.reserve(numbers.size() / 2);
    for (auto it = numbers.begin(); it != numbers.end(); std::advance(it, 2)) {
        int product = *it * *std::next(it);
        products.push_back(product);
    }
    
    return std::accumulate(products.begin(), products.end(), 0);
}

