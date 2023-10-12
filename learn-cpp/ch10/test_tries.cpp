#include <iostream>
#include <algorithm>

using namespace std;

#define MAX_N 10000
#define MAX_W 26

int tries[MAX_N][MAX_W];
int tidx = 0;
struct Value {
	int v;
};
Value values[MAX_N];
int valueIdx;


Value *tv[MAX_N];
void insert(char *name, Value *v) {
	int idx = 0;
	char *p = name;
	while (*p != '\0') {
		if (!tries[idx][*p - 'a']) {
			tries[idx][*p - 'a'] = ++tidx;
		}
		idx = tries[idx][*p - 'a'];
	}
	tv[idx] = v;
}

Value *find(char *name) {
	int idx = 0;
	char *p = name;
	while (*p != '\0') {
		int c = *p - 'a';
		if (!tries[idx][c]) return nullptr;
		idx = tries[idx][c];
	}
	return tv[idx];
}



int main() {

	return 0;
}
