#include "helper.h"

#ifdef __cplusplus
extern "C"
{
#endif
    bool has_enough_space(const char *dir, u_int8_t threshold)
    {
        std::error_code ec;
        const std::filesystem::space_info si = std::filesystem::space(dir, ec);
        if (ec.value() != 0)
        {
            std::cout << "Error: " << ec.message() << std::endl;
            exit(EXIT_FAILURE);
        }

        return static_cast<double>(si.available) / si.capacity > threshold / 100;
    }
#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
extern "C"
{
#endif
    size_t remove_files(const char* dir, const char *file_date)
    {
        std::cout << "remove " << file_date << std::endl;
        std::string str(file_date);   // 2021-0808
        size_t removed = 0;
        for (const auto& file_path : file_list(dir, std::regex(str))) {
            std::cout << file_path << std::endl;
            if (std::filesystem::remove(file_path)) removed++;
        }
        return removed;
    }
#ifdef __cplusplus
}
#endif