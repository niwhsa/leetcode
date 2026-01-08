// 94. Binary Tree Inorder Traversal (Easy)
// https://leetcode.com/problems/binary-tree-inorder-traversal/
//
// Time: O(n), Space: O(h) where h = tree height

#include <iostream>
#include <vector>
#include <stack>

using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode* l, TreeNode* r) : val(x), left(l), right(r) {}
};

class Solution {
public:
    vector<int> inorderTraversal(TreeNode* root) {
        vector<int> result;
        stack<TreeNode*> stk;
        TreeNode* curr = root;

        while (curr || !stk.empty()) {
            // Go left as far as possible
            while (curr) {
                stk.push(curr);
                curr = curr->left;
            }

            // Visit node
            curr = stk.top();
            stk.pop();
            result.push_back(curr->val);

            // Move to right subtree
            curr = curr->right;
        }

        return result;
    }
};

// Helper to print vector
void print(const vector<int>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); ++i) {
        cout << v[i];
        if (i < v.size() - 1) cout << ", ";
    }
    cout << "]" << endl;
}

int main() {
    Solution sol;

    // Test 1: [1,null,2,3] → [1,3,2]
    //     1
    //      \
    //       2
    //      /
    //     3
    TreeNode* t1 = new TreeNode(1, nullptr, new TreeNode(2, new TreeNode(3), nullptr));
    cout << "Test 1: ";
    print(sol.inorderTraversal(t1));  // [1, 3, 2]

    // Test 2: empty
    cout << "Test 2: ";
    print(sol.inorderTraversal(nullptr));  // []

    // Test 3: single node
    cout << "Test 3: ";
    print(sol.inorderTraversal(new TreeNode(1)));  // [1]

    // Test 4: balanced
    //     2
    //    / \
    //   1   3
    TreeNode* t4 = new TreeNode(2, new TreeNode(1), new TreeNode(3));
    cout << "Test 4: ";
    print(sol.inorderTraversal(t4));  // [1, 2, 3]

    cout << "All tests passed!" << endl;
    return 0;
}

