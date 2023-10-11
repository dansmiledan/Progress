#include <iostream>
#include <functional>

using namespace std;

bool check_size(const string &str, int sz) {
	return str.size() > sz;
}

ostream &print(ostream &os, const string &str) {
	return os << str;
}

int main() {
	auto check6 = bind(check_size, placeholders::_1, 6);
	string s = "hello world";
	cout << check6(s) << endl;

	vector<string> words = {"abc", "def"};
	ostream &os = cout;
	for_each(words.begin(), words.end(), [&os](const string &s) { os << s;});

	// error: os can not be copied
	// for_each(words.begin(), words.end(), bind(print, os, _1));
	// using follow:
	for_each(words.begin(), words.end(), bind(print, ref(os), placeholders::_1));


	return 0;
}
