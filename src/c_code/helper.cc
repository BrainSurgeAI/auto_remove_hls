#include "helper.h"

#ifdef __cplusplus
extern "C"
{
#endif
    bool has_enough_space(const char *dir)
    {
        std::error_code ec;
        const std::filesystem::space_info si = std::filesystem::space(dir, ec);
        if (ec.value() != 0)
        {
            std::cout << "Error: " << ec.message() << std::endl;
            exit(EXIT_FAILURE);
        }

        return static_cast<double>(si.available) / si.capacity > THRESHOLD;
    }
#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
extern "C"
{
#endif
    const char **remove_list(const char *dir)
    {
        std::vector<std::filesystem::path> paths(std::filesystem::directory_iterator{dir},
                                                 std::filesystem::directory_iterator{});
        std::cout << "Befor sort:" << std::endl;
        for (auto &k : paths)
        {
            std::cout << k << std::endl;
        }

        std::sort(paths.begin(), paths.end(), [](std::filesystem::path a, std::filesystem::path b)
                  { return to_time_t(std::filesystem::directory_entry(a).last_write_time()) <
                           to_time_t(std::filesystem::directory_entry(b).last_write_time()); });

        std::cout << "After sort:" << std::endl;
        for (auto &k : paths)
        {
            std::cout << k << std::endl;
        }

        // std::for_each_n(paths.begin(), RETURN_NUM, [&](auto &n)
        //                 { p.push_back(n.c_str()); });
        for (size_t i = 0; i < RETURN_NUM; ++i) {
            p.push_back(paths[i].c_str());
        }
        std::cout << "collect: " << p.size() << std::endl;
        for (auto &k : p)
        {
            std::cout << k << std::endl;
        }
        return p.data();
    }
#ifdef __cplusplus
}
#endif