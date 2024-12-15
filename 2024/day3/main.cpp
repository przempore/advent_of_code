#include "solution.hpp"

#include <fmt/core.h>
#include <fstream>
#include <sstream>

int main(int argc, char* argv[]) {
    if (argc < 2) {
        fmt::println("Usage {} path/to/input/file", argv[0]);
        return 1;
    }
    std::ifstream infile(argv[1], std::ios::in);
    if (not infile.is_open()) {
        fmt::println("File is not open.");
        return -1;
    }
    std::stringstream buffer;
    buffer << infile.rdbuf();
    fmt::println("part_1: {}", part_1(buffer.str()));

    return 0;
}

