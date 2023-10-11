#include <iostream>
#include <algorithm>

using namespace std;

int main() {
	auto p = [] { cout << "a  "; };
	p();

	vector<int> testData = {1, 2, 3, 4, 5, 6, 7, 8};

	auto it = testData.begin();
	while (it != testData.end()) {
		it = find_if(it, testData.end(), [](int t) { return t % 2 == 0;});
		cout << *it << endl;
		it++;
	}

	
	return 0;
}
