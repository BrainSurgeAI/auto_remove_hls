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