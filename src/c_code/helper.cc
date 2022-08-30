#include "helper.h"

#ifdef __cplusplus
extern "C"
{
#endif
    bool has_enough_space(const char *dir, u_int8_t threshold)
    {
        std::error_code ec;
        const std::filesystem::space_info si = std::filesystem::space(dir, ec);
        if (ec.value() != 0 ) {
            std::cout << "filesystem error: [" << ec.value() << "] " << ec.message() << std::endl;
            return true;
        }
        if (std::filesystem::is_empty(dir)) {
            std::cout << dir << " is empty\n"; 
            return true;
        }    
        
        std::cout << "available " << static_cast<double>(si.available) / si.capacity << std::endl; 
        return static_cast<double>(si.available) / si.capacity > threshold / 100;
    }
#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
extern "C"
{
#endif
    size_t remove_files(const char *dir, const char *file_date)
    {
        std::string str(file_date); // 2021-0808
        size_t removed = 0;
        for (const auto &file_path : file_list(dir, std::regex(str)))
        {
            std::error_code ec;
            if (std::filesystem::remove(file_path, ec))
            {
                removed++;
            }
            else
            {
                std::cerr << "remove" << file_path << " error " << ec.message() << std::endl;
            }
        }
        return removed;
    }
#ifdef __cplusplus
}
#endif