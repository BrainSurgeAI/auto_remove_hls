#include <iostream>
#include <filesystem>
#include <chrono>
#include <cstdint>
#include <algorithm>
#include <vector>
#include <string>

static const double THRESHOLD = 0.2;
using namespace std;

template <typename TP>
time_t to_time_t(TP tp)
{
    using namespace chrono;
    auto sctp = time_point_cast<system_clock::duration>(tp - TP::clock::now() + system_clock::now());
    return system_clock::to_time_t(sctp);
}

#ifdef __cplusplus
extern "C"
{
#endif
    bool has_enough_space(const char *dir);
#ifdef __cplusplus
}
#endif