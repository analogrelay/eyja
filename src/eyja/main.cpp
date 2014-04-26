#include <iostream>
#include <http.h>
#include "XGetopt.h"

// Arguments: "eyja -p:[port] -p
int main(int args, char** argv) {
	// Parse input arguments
	int c;
	int port = 8000;
	char* addr = nullptr;
	while ((c = getopt(args, argv, "p:a:")) != -1) {
		switch (c)
		{
		case 'p':
			port = atoi(optarg);
			break;
		case 'a':
			addr = optarg;
			break;
		}
	}

	std::cout << "[http] binding to " << (addr == nullptr ? "0.0.0.0" : addr) << ":" << port << std::endl;

	
}