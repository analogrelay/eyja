#include "http_server.h"
#include <http.h>

using namespace eyja;

http_server::http_server() {
	
}

http_server::~http_server() {
	HttpTerminate(HTTP_INITIALIZE_SERVER, nullptr);
}

int http_server::add_url(const wchar_t* url) {
	_urls.push_back(url);
}

int http_server::run() {
	int hr = HttpInitialize(HTTPAPI_VERSION_2, HTTP_INITIALIZE_SERVER, nullptr);
	if (FAILED(hr)) {
		return hr;
	}
	hr = HttpCreateServerSession(HTTPAPI_VERSION_2, &_sessionId, 0);
	if (FAILED(hr)) {
		return hr;
	}
	hr = HttpCreateUrlGroup(_sessionId, &_urlGroup, 0);
	if (FAILED(hr)) {
		return hr;
	}

	for (auto url : _urls) {
		hr = HttpAddUrlToUrlGroup(_urlGroup, url, 0, 0);
		if (FAILED(hr)) {
			return hr;
		}
	}

	SECURITY_ATTRIBUTES a;
	a.bInheritHandle = false;
	a.lpSecurityDescriptor = nullptr;
	a.nLength = sizeof(a);
	hr = HttpCreateRequestQueue(HTTPAPI_VERSION_2, L"eyja", &a, 0, &_requestQueue);
	if (FAILED(hr)) {
		return hr;
	}

	HTTP_BINDING_INFO binding;
	binding.RequestQueueHandle = _requestQueue;
	binding.Flags.Present = 1;
	hr = HttpSetUrlGroupProperty(_urlGroup, HttpServerBindingProperty, &binding, sizeof(binding));
	if (FAILED(hr)) {
		return hr;
	}

	return pump();
}

int http_server::pump() {
	HTTP_REQUEST req;
	//int hr = HttpReceiveHttpRequest(_requestQueue, HTTP_NULL_ID, 0, &req, )
}