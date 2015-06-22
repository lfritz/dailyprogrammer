#include <algorithm>
#include <array>
#include <cstdint>
#include <iostream>

constexpr int k = 8;
constexpr int l = 1000000;

int main() {
    std::array<int64_t, l> ys;
    ys[0] = 1;

    const std::array<int64_t, k> xs { 2, 3, 5, 7, 11, 13, 17, 19 };
    std::array<std::array<int64_t, l>::const_iterator, k> xi;
    std::array<int64_t, k> xv;
    for (int i = 0; i < k; ++i) {
        xi[i] = ys.cbegin();
        xv[i] = xs[i];
    }

    for (int i = 1; i < l; ++i) {
        int64_t n = xv[0];
        for (int j = 1; j < k; ++j)
            n = std::min(n, xv[j]);
        ys[i] = n;
        for (int j = 0; j < k; ++j)
            if (xv[j] == n)
                xv[j] = *(++xi[j]) * xs[j];
    }

    std::cout << ys[l-1] << "\n";
    return 0;
}
