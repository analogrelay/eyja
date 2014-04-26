#pragma once

#include "precompiled.h"

#include "value.h"

namespace eyja {
	class store {
	public:
		store();
		~store();

		// Set a value with the specified key
		// The store will COPY the buffer referred to by the input argument. The consumer RETAINS ownership of the value
		void set(const std::string key, const value& val);

		// Get a value with the specified key. The value contains a unique copy of the buffer, the consumer now OWNS that buffer.
		value get(const std::string key);
	private:
		class impl;
		impl* _impl;
	};
}