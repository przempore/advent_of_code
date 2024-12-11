#include <fmt/format.h>
#include <fmt/ranges.h>

#include <algorithm>
#include <fstream>
#include <numeric>
#include <sstream>
#include <string>
#include <vector>

void part_1(const std::string& file_path) {
    std::ifstream infile(file_path, std::ios::in);
    if (not infile.is_open()) {
        fmt::println("File is not open.");
        return;
    }
    std::string line;
    std::vector<int> a;
    std::vector<int> b;
    while (std::getline(infile, line)) {
        int i, j;
        std::istringstream iss(line);
        iss >> i >> j;
        a.push_back(i);
        b.push_back(j);
    }
    infile.close();

    std::ranges::sort(a);
    std::ranges::sort(b);

    std::vector<int> sums;
    for (int i = 0; i < a.size(); ++i) {
        sums.push_back(std::abs(a[i] - b[i]));
    }
    // fmt::print("sums: ");
    // fmt::print("{}", fmt::join(sums, ", "));
    // fmt::println("");
    fmt::println("sum: {}", std::accumulate(sums.begin(), sums.end(), 0));
}


void part_2(const std::string& file_path) {
    std::ifstream infile(file_path, std::ios::in);
    if (not infile.is_open()) {
        fmt::println("File is not open.");
        return;
    }
    std::string line;
    std::vector<int> a;
    std::vector<int> b;
    while (std::getline(infile, line)) {
        int i, j;
        std::istringstream iss(line);
        iss >> i >> j;
        a.push_back(i);
        b.push_back(j);
    }
    infile.close();

    std::vector<int> counts;
    for (int i = 0; i < a.size(); ++i) {
        counts.push_back(a[i] * std::count(b.begin(), b.end(), a[i]));
    }
    
    // fmt::print("counts: ");
    // fmt::print("{}", fmt::join(counts, ", "));

    fmt::println("part 2 sum: {}", std::accumulate(counts.begin(), counts.end(), 0));
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
