#include <iostream>
#include <functional>

using namespace std;

bool check_size(const string &str, int sz) {
	return str.size() > sz;
}

int main() {
	auto check6 = bind(check_size, placeholders::_1, 6);
	string s = "hello world";
	cout << check6(s) << endl;
	return 0;
}
