#include <cstring>
#include <memory>
#include <stdexcept>

#include "wrapper.h"
#include "counter-cpp/Counter.h"

using counter::Counter;

namespace {
    struct CounterInstance {
        std::unique_ptr<Counter> handle;
    };

    template <class T>
    T* Allocate(const std::vector<T>& input, size_t* length) {
        *length = input.size();

        size_t size = *length * sizeof(T);
        T* buf = static_cast<T*>(malloc(size));
        memcpy(buf, input.data(), size);

        return buf;
    }
}

extern "C" {
    void* CreateInstance(unsigned int initialValue) {
        auto ptr = std::unique_ptr<Counter>(new Counter(initialValue));
        void* instance = new CounterInstance{std::move(ptr)};
        return instance;
    }

    void ReleaseInstance(void* instance) {
        CounterInstance* counter = static_cast<CounterInstance*>(instance);
        delete counter;
    }

    int Increment(void* instance, unsigned int amount) {
        Counter* counter = static_cast<CounterInstance*>(instance)->handle.get();

        try {
            counter->Increment(amount);
            return 0;
        }
        catch (std::overflow_error& e) {
            // TODO: Store the exception message in a string for the FFI to retrieve
            return -1;
        }
    }

    int Decrement(void* instance, unsigned int amount) {
        Counter* counter = static_cast<CounterInstance*>(instance)->handle.get();

        try {
            counter->Decrement(amount);
            return 0;
        }
        catch (std::invalid_argument& e) {
            // TODO: Store the exception message in a string for the FFI to retrieve
            return -1;
        }
    }

    unsigned int GetCurrentValue(const void* instance) {
        const Counter* counter = static_cast<const CounterInstance*>(instance)->handle.get();
        return counter->GetCurrentValue();
    }

    unsigned int* GetHistory(const void* instance, size_t* length) {
        const Counter* counter = static_cast<const CounterInstance*>(instance)->handle.get();
        const std::vector<unsigned int>& history = counter->GetHistory();

        unsigned int* buffer = Allocate<unsigned int>(history, length);
        return buffer;
    }
} // extern "C"
