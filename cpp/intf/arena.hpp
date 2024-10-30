// Copyright 2024 Shane W. Mulcahy

#ifndef MEMORY_INTF_MEMORY_HPP_
#define MEMORY_INTF_MEMORY_HPP_

#include <cstddef>
#include <stdexcept>
#include <memory>

namespace atom {

    namespace mem {

        class Arena {

            private:
                std::unique_ptr<char[]> m_memory;
                std::size_t             m_capacity;
                std::size_t             m_offset;

            public:

        }
    } // namespace mem

} // namespace atom

#endif // MEMORY_INTF_MEMORY_HPP_