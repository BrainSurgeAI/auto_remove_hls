#include <iostream>
#include <filesystem>
#include <chrono>
#include <cstdint>
#include <algorithm>
#include <vector>
#include <string>
#include <regex>

static const double THRESHOLD = 0.2;

using namespace std;
namespace fs = std::filesystem;

template <typename TP>
time_t to_time_t(TP tp)
{
    using namespace chrono;
    auto sctp = time_point_cast<system_clock::duration>(tp - TP::clock::now() + system_clock::now());
    return system_clock::to_time_t(sctp);
}


std::vector<fs::path> file_list(fs::path dir, std::regex ext_pattern)
{
    std::vector<fs::path> result ;

    using iterator = std::conditional<false, fs::recursive_directory_iterator, fs::directory_iterator>::type;

    const iterator end;
    for( iterator iter { dir } ; iter != end ; ++iter )
    {
        // const std::string extension = iter->path().extension().string() ;
        //std::cout <<  iter->path().string() << std::endl;
        if ( fs::is_regular_file(*iter) && std::regex_search( iter->path().string(), ext_pattern ) ) 
            result.push_back( *iter ) ;
    }
    
    return result ;
}

#ifdef __cplusplus
extern "C"
{
#endif
    bool has_enough_space(const char *dir);
#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
extern "C"
{
#endif
    bool remove_files(const char* dir, const char* file_date);
#ifdef __cplusplus
}
#endif