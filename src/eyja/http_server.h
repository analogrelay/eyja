#include "precompiled.h"
#include <vector>

namespace eyja {
	class http_server {
	private:
		HTTP_SERVER_SESSION_ID _sessionId;
		HTTP_URL_GROUP_ID _urlGroup;
		HANDLE _requestQueue;

		std::vector<const wchar_t*> _urls;

		int pump();
	public:
		http_server();
		~http_server();

		int add_url(const wchar_t* url);
		int run();
	};
}