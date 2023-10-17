#include <iostream>
#include <algorithm>
#include <cstdlib>
#include <ctime>

const int N = 10;

template<int M>
struct TreeArray {
	int arr[M+1];

	void clear() {
		memset(arr, 0, sizeof(int) * (M+1));
	}

	// o(n)
	void build(int *data) {
		for (int i = 1; i <= M; i++) {
			arr[i] += data[i-1];
			int next = i + (-i & i);
			if (next <= M) {
				arr[next] += arr[i];
			}
		}
	}

	void update(int i, int diff) {
		int idx = i + 1;
		while (idx <= M) {
			arr[idx] += diff;
			idx += (-idx & idx);
		}
	}

	int log2n(int n) {
		int k = 0;
		while (n > 1) {
			k++;
			n >>= 1;
		}
		return k;
	}

	int nth(int n) {
		int s = 0, x = 0;
		for (int i = log2n(M); i >= 0; i--) {
			x += 1 << i;
			if (x >= M || s + arr[x] >= n) {
				x -= 1 << i;
			} else {
				s += arr[x];
			}
		}
		return x;
	}

};

int main() {
	int data[N];
	srand(time(nullptr));
	for (int i = 0; i < N; i++) {
		data[i] = rand() % 2;
		std::cout << data[i] << " ";
	}
	std::cout << std::endl;
	TreeArray<N> arr1;
	arr1.clear();
	TreeArray<N> arr2;
	arr2.clear();
	arr1.build(data);
	for (int i = 0; i < N; i++) {
		if (data[i]) {
			arr2.update(i, data[i]);
		}
	}

	for (int i = 1; i <= N; i++) {
		assert(arr1.arr[i] == arr2.arr[i]);
		std::cout << arr1.arr[i] << " ";
	}
	std::cout << std::endl;

	for (int i = 0; i <= N; i++) {
		std::cout << i << " " << arr1.nth(i) << std::endl;
	}

	return 0;
}
