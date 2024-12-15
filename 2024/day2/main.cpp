#include <fmt/core.h>
#include <fmt/ranges.h>

#include <algorithm>
#include <fstream>
#include <optional>
#include <sstream>
#include <string>
#include <vector>

bool is_safe(std::vector<int> const& numbers) {
    std::optional<bool> is_negative{std::nullopt};
    int permissed = 0;
    for (auto it = std::next(numbers.begin()); it != numbers.end(); ++it) {
        int diff = *std::prev(it) - *it;
        if (diff == 0) return false;
        if (not is_negative.has_value()) {
            is_negative = diff < 0;
        }

        if (std::abs(diff) > 3) {
            // fmt::println("Should be at most 3 but it's {}", diff);
            if (permissed++ > 1) return false;
        }
        if (is_negative.value()) {
            if (diff > 0) {
                // fmt::println("Should be negatve but it's {}", diff);
                return false;
            }
        } else {
            if (diff < 0) {
                // fmt::println("Should be positive but it's {}", diff);
                return false;
            }
        }
    }
    return true;
}

std::vector<bool> is_safe_vec(std::vector<int> const& numbers) {
    std::vector<bool> safe_values;
    safe_values.reserve(numbers.size());
    std::optional<bool> is_negative{std::nullopt};
    for (auto it = std::next(numbers.begin()); it != numbers.end(); ++it) {
        int diff = *std::prev(it) - *it;
        if (diff == 0) {
            safe_values.push_back(false);
            continue;
        }
        if (not is_negative.has_value()) {
            is_negative = diff < 0;
        }

        if (std::abs(diff) > 3) {
            safe_values.push_back(false);
            continue;
        }
        if (is_negative.value()) {
            if (diff > 0) {
                safe_values.push_back(false);
                continue;
            }
        } else {
            if (diff < 0) {
                safe_values.push_back(false);
                continue;
            }
        }
        safe_values.push_back(true);
    }
    // fmt::println("{}", fmt::join(safe_values, ", "));
    return safe_values;
}

void part_1(std::string const& file_path) {
    std::ifstream infile(file_path, std::ios::in);
    if (not infile.is_open()) {
        fmt::println("File is not open.");
        return;
    }

    std::vector<std::vector<int>> numbers;
    std::string line{};
    while (std::getline(infile, line)) {
        std::stringstream ss(line);

        std::vector<int> row_values;
        int value;

        while (ss >> value) {
            row_values.push_back(value);
        }

        if (not row_values.empty()) {
            numbers.push_back(row_values);
        }
    }

    infile.close();

    int safe_count{0};
    for (auto& n : numbers) {
        if (std::ranges::all_of(is_safe_vec(n),
                                [](auto a) { return a == true; }))
            safe_count++;
    }

    fmt::println("Safe count: {}", safe_count);
}

void part_2(std::string const& file_path) {
    std::ifstream infile(file_path, std::ios::in);
    if (not infile.is_open()) {
        fmt::println("File is not open.");
        return;
    }

    std::vector<std::vector<int>> numbers;
    std::string line{};
    while (std::getline(infile, line)) {
        std::stringstream ss(line);

        std::vector<int> row_values;
        int value;

        while (ss >> value) {
            row_values.push_back(value);
        }

        if (not row_values.empty()) {
            numbers.push_back(row_values);
        }
    }

    infile.close();

    int safe_count{0};
    for (auto& n : numbers) {
        // auto count = std::ranges::count_if(is_safe_vec(n),
        //                                    [](auto a) { return a == false; });
        // if (count <= 1) {
        if (is_safe(n)) {
            // fmt::println("count <= 1: {}", count);
            safe_count++;
        }
    }

    fmt::println("Safe count: {}", safe_count);
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        fmt::println("Usage {} path/to/input/file", argv[0]);
        return 1;
    }
    part_1(argv[1]);
    part_2(argv[1]);

    return 0;
}

