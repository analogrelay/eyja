#include "value.h"

using namespace eyja;
using namespace std;

const value value::empty;

value::value() : _type(value_type::Null), _buffer(nullptr) {}
value::value(value_type type, std::shared_ptr<uint8_t[]> buffer, size_t size) : _type(type), _buffer(buffer), _size(size) {}

const value_type value::type() const {
	return _type;
}

const std::shared_ptr<uint8_t[]> value::buffer() const {
	return _buffer;
}

const size_t value::size() const {
	return _size;
}