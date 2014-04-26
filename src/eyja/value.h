#pragma once

#include "precompiled.h"

#include <memory>
#include <vector>

namespace eyja {
	enum class value_type {
		Null,
		Utf8String
	};

	class value {
	public:
		value(value_type type, std::shared_ptr<uint8_t[]> buffer, size_t size);
		
		// Returns the type of this value
		const value_type type() const;

		// Returns a weak pointer to the buffer. The buffer will be destroyed if the source value gets destroyed.
		const std::shared_ptr<uint8_t[]> buffer() const;

		// Returns the size, in bytes, of the buffer
		const size_t size() const;

		static const value empty;
	private:
		value();
		
		value_type _type;
		std::shared_ptr<uint8_t[]> _buffer;
		size_t _size;
	};
}