#include <iostream>
#include <cstdlib>
#include <ctime>
#include <queue>

#define MAX_N 150000
#define MAX_PRI (1e7 + 7)

struct Value {
	int x, y, z;
};

struct Node {
	int key, pri;
	Node *left, *right;
	Value *v;
	int size;
	int level;
};

// max heap
struct Treap {
	Value values[MAX_N];
	int vIdx;
	Node nodes[MAX_N];
	int nIdx;
	int key;
	Node *root;

	Treap():vIdx(0), nIdx(0) {
		key = 0;
		root = nullptr;
	}

	Node *newNode() {
		Node *ret = &nodes[nIdx++];
		Value *v = &values[vIdx++];
		v->x = v->y = v->z = rand();
		ret->v = v;
		ret->pri = rand() % (int)MAX_PRI;
		ret->key = key++;
		ret->left = nullptr;
		ret->right = nullptr;
		return ret;
	}

	/**
             A           B
			/ \         / \
		   B   C       D   A
          / \             / \ 
		 D   E           E   C
	*/ 
	Node *rotateRight(Node *node) {
		Node *l = node->left;
		node->left = l->right;
		l->right = node;
		return l;
	}

	Node *rotateLeft(Node *node) {
		Node *r = node->right;
		node->right = r->left;
		r->left = node;
		return r;
	}

	Node *insertNode(Node *root, Node *node) {
		if (root == nullptr) {
			return node;
		}
		if (node->key < root->key) {
			root->left = insertNode(root->left, node);
			if (root->pri < root->left->pri) {
				return rotateRight(root);
			}
		} else if (node->key > root->key) {
			root->right = insertNode(root->right, node);
			if (root->pri < root->right->pri) {
				return rotateLeft(root);
			}
		}
		return root;
	}

	Node *find(Node *root, Node *node) {
		if (root == nullptr) {
			return root;
		}
		if (node->key < root->key) {
			return find(root->left, node);
		} else if (node->key > root->key) {
			return find(root->right, node);
		}
		return root;
	}

	Node *deleteNode(Node *root, Node *node) {
		if (root == nullptr) {
			return root;
		}
		if (node->key < root->key) {
			root->left = deleteNode(root->left, node);
		} else if (node->key > root->key) {
			root->right = deleteNode(root->right, node);
		} else if (root->left == nullptr) {
			return root->right;
		} else if (root->right == nullptr) {
			return root->left;
		} else if (root->left->pri < root->right->pri){
			root = rotateLeft(root);
			root->left = deleteNode(root->left, node);
		} else {
			root = rotateRight(root);
			root->right = deleteNode(root->right, node);
		}
		return root;
	}
};

int getSize(Node *node) {
	if (node == nullptr) {
		return 0;
	}
	node->size = 1 + getSize(node->left) + getSize(node->right);
	return node->size;
}

int main() {
	srand(time(nullptr));
	Treap treap;
	for (int i = 0; i < MAX_N; i++) {
		Node *newNode = treap.newNode();
		treap.root = treap.insertNode(treap.root, newNode);
	}
	getSize(treap.root);
	std::queue<Node*> que;
	treap.root->level = 0;
	que.push(treap.root);
	while(!que.empty()) {
		Node *f = que.front();
		std::cout << f->level << " " << f->size << std::endl;
		que.pop();
		if (f->left) {
			f->left->level = f->level + 1;
			que.push(f->left);
		}
		if(f->right) {
			f->right->level = f->level + 1;
			que.push(f->right);
		}
	}
	return 0;
}
