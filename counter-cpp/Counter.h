#ifndef COUNTER_H
#define COUNTER_H

#include <vector>

namespace counter {

class Counter {
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
