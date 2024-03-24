#include "Counter.h"

#include <limits>
#include <stdexcept>

namespace counter {

Counter::Counter(unsigned int initialValue)
    : value{initialValue},
      history{std::vector<unsigned int>()}
{
    this->history.push_back(initialValue);
}

void Counter::Increment(unsigned int amount) {
    if (amount > (std::numeric_limits<unsigned int>::max() - this->value)) {
        throw std::overflow_error("Adding this amount would overflow the counter");
    }

    this->value += amount;
    history.push_back(this->value);
}

void Counter::Decrement(unsigned int amount) {
    if (amount > this->value) {
        throw std::invalid_argument("Amount must be no bigger than current value");
    }

    this->value -= amount;
    history.push_back(this->value);
}

unsigned int Counter::GetCurrentValue() const noexcept {
    return this->value;
}

const std::vector<unsigned int>& Counter::GetHistory() const noexcept {
    return this->history;
}

} // namespace counter
