#include "store.h"

#include <map>

using namespace eyja;

class store::impl {
public:
};

store::store() : _impl(new impl()) {}
store::~store() { delete _impl; _impl = nullptr; }

void store::set(const std::string key, const value& val) {
}

value store::get(const std::string key) {
	return value::empty;
}