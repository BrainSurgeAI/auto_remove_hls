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
    bool remove_files(const char* dir, const char *file_date)
    {
        std::string str(file_date);
        const std::regex date_regex("20210808");

        for (const auto& file_path : file_list(dir, date_regex)) {
            std::cout << file_path << std::endl;
            std::filesystem::remove(file_path);
        }

        return true;
    }
#ifdef __cplusplus
}
#endif