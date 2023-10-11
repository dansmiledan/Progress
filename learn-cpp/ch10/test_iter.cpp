#include <iostream>
#include <algorithm>
#include <list>

int main() {

	// 10.27
	std::string s1 {"The      string    with many       sppppppppeeeeeeaces!"};
    std::cout << "before: " << s1 << '\n';
 
	std::list<char> l;
    std::unique_copy(s1.begin(), s1.end(), std::back_inserter(l));
                     //[](char c1, char c2) { return c1 == ' ' && c2 == ' '; });
 
	for (auto it = l.begin(); it != l.end(); it++) {
		std::cout << *it;
	}
	std::cout << std::endl;

	return 0;
}
