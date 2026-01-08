# LeetCode Workflow

## File Naming Convention

```
<problem_number>_<problem_title_snake_case>.<ext>
```

Examples:
- `0001_two_sum.cpp`
- `0094_binary_tree_inorder_traversal.rs`

## Adding a New Solution

### C++

```bash
cd ~/personal/leetcode/cpp
nvim 0001_two_sum.cpp
```

Template:
```cpp
// 1. Two Sum (Easy)
// https://leetcode.com/problems/two-sum/
//
// Time: O(?), Space: O(?)

#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    // Solution here
};

int main() {
    Solution sol;
    // Tests here
    cout << "All tests passed!" << endl;
    return 0;
}
```

Compile & run:
```bash
g++ -std=c++20 -O2 -Wall 0001_two_sum.cpp -o 0001 && ./0001
```

Or in Neovim: `<leader>rr`

### Rust

```bash
cd ~/personal/leetcode/rust
nvim 0001_two_sum.rs
```

Template:
```rust
// 1. Two Sum (Easy)
// https://leetcode.com/problems/two-sum/
//
// Time: O(?), Space: O(?)

pub struct Solution;

impl Solution {
    // Solution here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Tests here
    }
}
```

Run tests:
```bash
rustc --test 0001_two_sum.rs && ./0001_two_sum
```

## Committing

```bash
cd ~/personal/leetcode
git add .
git commit -m "Add <problem_number> <Problem Title>"
git push
```

## Neovim Keymaps

| Key | Action |
|-----|--------|
| `<leader>rr` | Compile & run current file |
| `<leader>b` | Toggle breakpoint |
| `<F5>` | Start debugger |
| `gl` | Show diagnostic (error details) |
| `gd` | Go to definition |
| `K` | Hover documentation |

## Useful Resources

- [LeetCode](https://leetcode.com/)
- [NeetCode Roadmap](https://neetcode.io/roadmap)
- [C++ Reference](https://en.cppreference.com/)
- [Rust Book](https://doc.rust-lang.org/book/)

