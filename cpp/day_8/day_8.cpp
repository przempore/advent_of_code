#include <algorithm>
#include <cstdint>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <iterator>
#include <limits>
#include <optional>
#include <ranges>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

struct Content {
    using Network =
        std::unordered_map<std::string, std::pair<std::string, std::string>>;
    std::string instruction{""};
    Network network{};
    std::string current_node{""};
    int current_instruction_index{0};
};

std::ostream& operator<<(std::ostream& os, const Content& content) {
    os << "Content: " << std::endl;
    os << "\tInstruction: " << content.instruction << std::endl;
    os << "\tNetwork: " << std::endl;
    for (const auto& [key, value] : content.network) {
        os << "\t\t" << key << " -> {" << value.first << " " << value.second
           << "}" << std::endl;
    }
    os << "\tCurrent node: " << content.current_node << std::endl;
    return os;
}

std::string remove_all_whitespaces(std::string input) {
    input.erase(std::remove_if(input.begin(), input.end(), ::isspace),
                input.end());
    return input;
}

std::pair<std::string, std::pair<std::string, std::string>> parse_line(
    const std::string& line) {
    std::string line_without_whitespaces = remove_all_whitespaces(line);

    std::pair<std::string, std::pair<std::string, std::string>> result;

    const size_t equal_sign_pos = line_without_whitespaces.find('=');
    result.first = line_without_whitespaces.substr(0, equal_sign_pos);
    const size_t first_comma_pos = line_without_whitespaces.find(',');
    result.second.first =
        line_without_whitespaces.substr(equal_sign_pos + 2, 3);
    result.second.second =
        line_without_whitespaces.substr(first_comma_pos + 1, 3);

    return result;
}

std::optional<Content> parse_file(const std::string& filename) {
    std::ifstream file(filename);

    if (not file.is_open()) {
        std::cerr << "Could not open file: " << filename << std::endl;
        return std::nullopt;
    }

    Content content;
    std::getline(file, content.instruction);

    std::string line;
    std::getline(file, line);
    if (line != "") {
        std::cerr << "Expected empty line after instruction, but got \"" << line
                  << '\"' << std::endl;
        return std::nullopt;
    }

    while (std::getline(file, line)) {
        content.network.insert(parse_line(line));
    }

    return content;
}

std::optional<int> part1(Content& content) {
    int64_t steps{0};
    int instruction_idx{0};
    content.current_node = "AAA";
    while (content.current_node != "ZZZ") {
        char instruction =
            content.instruction[instruction_idx % content.instruction.size()];
        instruction_idx++;

        if (instruction == 'L') {
            content.current_node = content.network[content.current_node].first;
        } else if (instruction == 'R') {
            content.current_node = content.network[content.current_node].second;
        } else {
            std::cerr << "Unknown instruction: " << instruction << std::endl;
            return std::nullopt;
        }
        steps++;
    }

    return steps;
}

std::optional<int> part2(Content& content) {
    Content::Network current_nodes_pairs;
    std::ranges::copy_if(
        content.network,
        std::inserter(current_nodes_pairs, current_nodes_pairs.begin()),
        [](auto& v) { return v.first.back() == 'A'; });

    std::vector<std::string> current_nodes;
    std::ranges::transform(current_nodes_pairs,
                           std::back_inserter(current_nodes),
                           [](auto& v) { return v.first; });

    int64_t steps{0};
    int instruction_idx{0};
    while (std::ranges::any_of(current_nodes,
                               [](auto& v) { return v.back() != 'Z'; })) {
        const char instruction =
            content.instruction[instruction_idx % content.instruction.size()];
        instruction_idx++;
        for (auto& v : current_nodes) {
            if (instruction == 'L') {
                v = content.network[v].first;
            } else if (instruction == 'R') {
                v = content.network[v].second;
            } else {
                std::cerr << "Unknown instruction: " << instruction
                          << std::endl;
            }
        };

        if (steps >= std::numeric_limits<int64_t>::max()) {
            return steps;
        }

        steps++;
    }

    return steps;
}

int main(int argc, char** argv) {
    std::optional<Content> example_1 = parse_file("input_example.txt");
    if (not example_1.has_value()) {
        return 1;
    }
    std::cout << "Part 1: Example steps: "
              << part1(example_1.value()).value_or(0) << std::endl;
    std::optional<Content> example_2 = parse_file("input_example_2.txt");
    if (not example_2.has_value()) {
        return 1;
    }
    std::cout << "Part 1: Example steps: "
              << part1(example_2.value()).value_or(0) << std::endl;

    std::optional<Content> part1_content = parse_file("input_part1.txt");
    if (not part1_content.has_value()) {
        return 1;
    }
    std::cout << "Part 1 steps: " << part1(part1_content.value()).value_or(0)
              << std::endl;

    std::optional<Content> part2_example_content =
        parse_file("part2_example.txt");
    if (not part2_example_content.has_value()) {
        return 1;
    }
    std::cout << "Part 2: Example steps: "
              << part2(part2_example_content.value()).value_or(0) << std::endl;

    std::optional<Content> part2_input = parse_file("input_part1.txt");
    if (not part2_input.has_value()) {
        return 1;
    }
    std::cout << "Part 2 steps: " << part2(part2_input.value()).value_or(0)
              << std::endl;

    return 0;
}
