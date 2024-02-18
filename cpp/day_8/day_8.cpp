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
};

std::ostream& operator<<(std::ostream& os, const Content& content) {
    os << "Content: " << std::endl;
    os << "\tInstruction: " << content.instruction << std::endl;
    os << "\tNetwork: " << std::endl;
    for (const auto& [key, value] : content.network) {
        os << "\t\t" << key << " -> {" << value.first << " " << value.second
           << "}" << std::endl;
    }
    return os;
}

std::pair<std::string, std::pair<std::string, std::string>> parse_line(
    std::string& line) {
    std::ranges::remove_if(line, ::isspace);
    const size_t equal_sign_pos = line.find('=');
    return {line.substr(0, equal_sign_pos),
            {line.substr(equal_sign_pos + 2, 3),
             line.substr(line.find(',') + 1, 3)}};
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

int part1(const Content& content) {
    int64_t steps{0};
    int instruction_idx{0};
    std::string current_node = "AAA";
    while (current_node != "ZZZ") {
        char instruction =
            content.instruction[instruction_idx % content.instruction.size()];
        instruction_idx++;

        if (instruction == 'L') {
            current_node = content.network.at(current_node).first;
        } else {
            current_node = content.network.at(current_node).second;
        }
        steps++;
    }

    return steps;
}

std::vector<std::string> get_nodes(const Content& content) {
    Content::Network current_nodes_pairs;
    std::ranges::copy_if(
        content.network,
        std::inserter(current_nodes_pairs, current_nodes_pairs.begin()),
        [](auto& v) { return v.first.back() == 'A'; });

    std::vector<std::string> current_nodes;
    std::ranges::transform(current_nodes_pairs,
                           std::back_inserter(current_nodes),
                           [](auto& v) { return v.first; });

    return current_nodes;
}

bool all_ends_with_Z(const std::vector<std::string>& nodes) {
    // std::ranges::copy(nodes,
    //                   std::ostream_iterator<std::string>(std::cout, " "));
    // std::cout << std::endl;

    return std::ranges::all_of(nodes, [](auto& v) { return v.back() == 'Z'; });
}

int part2(const Content& content) {
    auto current_nodes = get_nodes(content);

    int64_t steps{0};
    int instruction_idx{0};
    while (not all_ends_with_Z(current_nodes)) {
        const char instruction =
            content.instruction[instruction_idx % content.instruction.size()];
        instruction_idx++;
        for (auto& v : current_nodes) {
            if (instruction == 'L') {
                v = content.network.at(v).first;
            } else {
                v = content.network.at(v).second;
            }
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
    std::cout << "Part 1: Example steps: " << part1(example_1.value())
              << std::endl;
    std::optional<Content> example_2 = parse_file("input_example_2.txt");
    if (not example_2.has_value()) {
        return 1;
    }
    std::cout << "Part 1: Example steps: " << part1(example_2.value())
              << std::endl;

    std::optional<Content> part1_content = parse_file("input_part1.txt");
    if (not part1_content.has_value()) {
        return 1;
    }
    std::cout << "Part 1 steps: " << part1(part1_content.value()) << std::endl;

    std::optional<Content> part2_example_content =
        parse_file("part2_example.txt");
    if (not part2_example_content.has_value()) {
        return 1;
    }
    std::cout << "Part 2: Example steps: "
              << part2(part2_example_content.value()) << std::endl;

    std::optional<Content> part2_input = parse_file("input_part1.txt");
    if (not part2_input.has_value()) {
        return 1;
    }
    std::cout << "Part 2 steps: " << part2(part2_input.value()) << std::endl;

    return 0;
}
