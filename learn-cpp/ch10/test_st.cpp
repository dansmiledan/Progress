#include <iostream>
#include <cstdlib>
#include <ctime>

using namespace std;
const int MAX_LEN = 100;
int logn[MAX_LEN];
const int logmax = 8;

// pre logn
void pre() {
	logn[1] = 0;
	logn[2] = 1;
	for (int i = 3; i < MAX_LEN; i++) {
		logn[i] = logn[i / 2] + 1;
		cout << i << ": " << logn[i] << endl;
	}
}


int main() {
	pre();
	srand(time(nullptr));
	int randData[MAX_LEN];
	int stData[MAX_LEN][logmax];
	for (int i = 0; i < MAX_LEN; i++) {
		randData[i] = rand() % MAX_LEN;
		stData[i][0] = randData[i];
		cout << randData[i] << " ";
	}
	cout << endl;
	for (int j = 1; j < logmax; j++) {
		for (int i = 0; i + (1 << j) - 1 < MAX_LEN; i++) {
			stData[i][j] = max(stData[i][j-1], stData[i + (1 << (j-1))][j-1]);
		}
	}
	/*
	st[i][j] -> [i, i + 2^j -1]  [i, i + 2^(j-1) -1] [i + 2^(j-1), i + 2^(j-1) + 2^(j-1)-1]
	st[i][j] = max(st[i][j-1], st[i + 2^(j-1)][j-1])
	[0][3] 0 <-> 8   7<->15
	0-15
	0 0  [0, 0 + 2^0 - 1] -> [0, 0]
	0 1  [0, 0 + 2^1 - 1] -> [0, 1]
	0 2  [0, 0 + 2^2 - 1] -> [0, 3]  [4 - 4 + 1, 1 + 4 - 1] [1, 4]
	0 4  [0, 0 + 2^4 - 1] -> [0, 15]
	*/
	for (int i = 0; i < 5; i++) {
		for (int j = i; j < 5; j++) {
			int s = logn[j - i]; // [0, 3]
			cout << "[" << i << "," << j << "]: " << max(stData[i][s], stData[j- (1 << s)+1][s]) << endl; 
		}
	}
	
	return 0;
}
