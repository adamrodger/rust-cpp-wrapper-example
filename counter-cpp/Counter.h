#ifndef COUNTER_H
#define COUNTER_H

#ifdef _WIN32
#  define EXPORT __declspec(dllexport)
#elif __linux__
#  define EXPORT
#else
#  error Unsupported platform
#endif

#include <vector>

namespace counter {

class EXPORT Counter {
public:
    Counter(unsigned int initialValue);

    void Increment(unsigned int amount);
    void Decrement(unsigned int amount);

    unsigned int GetCurrentValue() const noexcept;
    const std::vector<unsigned int>& GetHistory() const noexcept;

private:
    unsigned int value{};
    std::vector<unsigned int> history{};
}; // class Counter

} // namespace counter

#endif
