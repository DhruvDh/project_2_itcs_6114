# ITCS 6114 - Project 2 - Question 1

## Introduction

I've used a Red-Black tree to implement a balanced binary search tree. There are comments throughout the code to explain my implementation. Here I'll only focus on how I counted anagrams, and also my logic for checking whether it is a binary heap, since I don't think I need to explain how or why Red Black trees work.

## Checking if Binary Search Tree produced is also a Max Binary Heap

A balanced BST can be equivalent to a Max binary heap only if there are only 2 nodes in the tree, and if the parent is greater than the child node.

In all other cases, the fact that the right child of a BST node is greater than it's own parent makes it impossible for it to be equivalent to a Max binary heap, which requires the parent to be larger than it's children.

Considering this, the code for checking equivalence is as follows -

```
if nodes.len() == 2 {
    if nodes[1] < nodes[0] {
        println!("Yes, it is equivalent to a Max binary heap.");
    } else {
        println!(
            "It is not a Max binary heap because {:?} is > than {:?}.",
            nodes[1].0, nodes[0].0
        );
    }
} else {
    println!("No, it is not a Max binary heap.");
}
```

## Counting Anagrams

I count anagrams by comparing each word in the tree with every other word in the tree -

- First I check if they have the same length, if so I do further checking.
    - For each word in the tree, I first find the individual characters in the word -  
    ```
    "king" => ['k', 'i', 'n', 'g']
    ``` 
    For each individual character in `['k', 'i', 'n', 'g']`, I check whether the word I am comparing to also has the same character.
    - If so I check whether it also has the rest of the characters in 'king'. If it does, yet it is an anagram, and I update the count.
    - If I encounter a character that is not in the word, I break the loop and move on to the next word, since it cannot be an anagram.
- If they're not the same length, they cannot be anagrams of each other, and so I move on to the next word.

At the end, I substract 1 from the final anagram counts, since this algorithm counts each word to be an anagram of itself.

The exact implementation is as follows -

Here `words.keys()` is a copy of the keys inside the tree.

```
for node in tree.iter_mut() {
    for word in words.keys() {
        // for each character in the word we're finding the anagram for
        let mut is_anagram = true;

        // only if the word lenghts are the same then they can be anagrams
        if word.len() == node.0.len() {
            // check if it's an anagram by checking if it also contains all letters
        for _char in node.0.to_lowercase().chars() {
            // check if element also contains the character
            if word.to_lowercase().contains(_char) {
                continue; // if so, continue
            } else {
                is_anagram = false;
                break; // else, this is not an anagram, so break. since we need all characters for it to be an anagram.
            }
        }

    // if all characters are the same, is_anagram will still be true (it's default value).
    if is_anagram {
        *node.1 = *node.1 + 1;
    }
}
```