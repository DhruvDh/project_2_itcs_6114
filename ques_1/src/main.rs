mod lib;
use std::io::stdin;

fn main() {
    let mut tree = lib::Tree::new();

    println!("Enter a sentence and I'll insert the words into a Binary search tree..");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("I couldn't read that :(");

    let input = input.trim();

    // insert each word into the tree
    if input.len() > 0 {
        for word in input.split_whitespace() {
            tree.insert(String::from(word), 0);
        }
    } else {
        println!("String was empty");
    }

    println!("\n\n");
    println!("This tree's size is {:?} ", tree.len());
    tree.print_tree();

    println!("\nThis is the tree I made. What would you like to do now?");

    // loop the menu forever until user decided to quit
    loop {
        println!("\n\t1. Create a new Binary search tree.");
        println!("\t2. Find lenght of current tree.");
        println!("\t3. Add an word to the binary search tree.");
        println!("\t4. Delete a word from the binary search tree.");
        println!("\t5. Print the elements of the tree.");
        println!("\t6. Check if it is also a Max Binary heap.");
        println!("\t7. Find anagrams for each word in the BST.");
        println!("\t8. Exit.");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("I couldn't read that :(");

        let input = input.trim();

        println!("\n\n");
        match input.parse::<usize>() {
            // if selected option is 1
            Ok(1) => {
                tree = lib::Tree::new();

                println!("Enter a sentence and I'll insert the words into a Binary search tree..");
                let mut input = String::new();
                stdin()
                    .read_line(&mut input)
                    .expect("I couldn't read that :(");

                let input = input.trim();

                // insert each word into the tree
                if input.len() > 0 {
                    for word in input.split_whitespace() {
                        tree.insert(String::from(word), 0);
                    }
                } else {
                    println!("String was empty");
                }

                println!("\n\n");
                println!("This tree's size is {:?} ", tree.len());

                tree.print_tree();

                println!("\nThis is the tree I made. What would you like to do now?");
            }

            // 2. Find lenght of current tree.
            Ok(2) => {
                println!(
                    "My interpretation of length is size of the tree - 1. Assuming that is true -",
                );
                println!("Length (Size - 1) of the tree is {}", tree.len() - 1);
                println!("I'll also print the tree again for your reference.");
                tree.print_tree();
            }

            // 3. Add an word to the binary search tree.
            Ok(3) => {
                println!("Enter the word you'd like to insert -");
                let mut input = String::new();
                stdin()
                    .read_line(&mut input)
                    .expect("I couldn't read that :(");

                let input = input.trim();
                tree.insert(String::from(input), 0);

                println!("\n I've successfully added {}. The tree is now -", input);
                tree.print_tree();
            }

            // 4. Delete a word from the binary search tree.
            Ok(4) => {
                println!("I'll print the tree again for your reference.");
                tree.print_tree();
                println!("Enter the word you'd like to remove -");
                let mut input = String::new();
                stdin()
                    .read_line(&mut input)
                    .expect("I couldn't read that :(");

                let input = input.trim();
                match tree.remove(&String::from(input)) {
                    Some(_n) => {
                        println!("\n I've removed {} successfully. The tree is now -", input);
                        tree.print_tree();
                    }
                    _ => {
                        println!("I couldn't find {} in the tree.", input);
                    }
                };
            }

            // 5. Print the elements of the tree.
            Ok(5) => {
                println!("The elements of the tree are -\n");
                println!("{:?}", tree.keys());
            }

            // 6. Check if it is also a Max Binary heap.
            Ok(6) => {
                // cloning to ensure no pointers are not messed up accidentally
                let _nodes = tree.clone();
                let _nodes = _nodes.iter();
                let mut nodes = vec![];

                for node in _nodes {
                    nodes.push(node);
                }

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
            }

            // 7. Find anagrams for each word in the BST.
            Ok(7) => {
                let words = tree.clone();

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
                    }
                }

                println!("\nThe anagram counts are -");
                println!("(I am considering the same words to be anagrams of each other.. Also, checking whether it is an anagram is case-insensitive.)");
                for pair in tree.keys().zip(tree.values()) {
                    // -1 because program counted words to be anagrams of themselves
                    println!("{}: {}", pair.0, pair.1 - 1);
                }
            }

            Ok(8) => std::process::exit(0),
            _ => {}
        }
    }
}

mod tests {
    use super::lib;

    // test for 1.
    #[test]
    fn test_new_tree() {
        let mut tree = lib::Tree::new();

        println!("Enter a sentence and I'll insert the words into a Binary search tree..");
        let input = String::from("There has to be a better way to do this");

        // insert each word into the tree
        if input.len() > 0 {
            for word in input.split_whitespace() {
                tree.insert(String::from(word), 0);
            }
        } else {
            println!("String was empty");
        }

        println!("\n\n");
        tree.print_tree();

        println!("\nThis is the tree I made. What would you like to do now?");
    }

    // test for 2.
    #[test]
    fn test_len() {
        let mut tree = lib::Tree::new();
        let input = String::from("There has to be a better way to do this");

        for word in input.split_whitespace() {
            tree.insert(String::from(word), 0);
        }

        println!("My interpretation of length is size of the tree - 1. Assuming that is true -",);
        println!("Length (Size - 1) of the tree is {}", tree.len() - 1);
        println!("I'll also print the tree again for your reference.");
        tree.print_tree();
    }

    // test for 3.
    #[test]
    fn test_add() {
        let mut tree = lib::Tree::new();
        let input = String::from("There has to be a better way to do this");

        for word in input.split_whitespace() {
            tree.insert(String::from(word), 0);
        }

        println!("My interpretation of length is size of the tree - 1. Assuming that is true -",);
        println!("Length (Size - 1) of the tree is {}", tree.len() - 1);
        println!("I'll also print the tree again for your reference.");
        tree.print_tree();

        tree.insert(String::from("word"), 0);
    }

    // test for 4.
    #[test]
    fn test_remove() {
        let mut tree = lib::Tree::new();
        let input = String::from("There has to be a better way to do this");

        for word in input.split_whitespace() {
            tree.insert(String::from(word), 0);
        }

        match tree.remove(&String::from("word")) {
            Some(_n) => {
                println!("\n I've removed {} successfully. The tree is now -", input);
                tree.print_tree();
            }
            _ => {
                println!("I couldn't find {} in the tree.", input);
            }
        };

        match tree.remove(&String::from("better")) {
            Some(_n) => {
                println!("\n I've removed {} successfully. The tree is now -", input);
                tree.print_tree();
            }
            _ => {
                println!("I couldn't find {} in the tree.", input);
            }
        };
    }

    // test for 5.
    #[test]
    fn test_print() {
        let mut tree = lib::Tree::new();
        let input = String::from("There has to be a better way to do this");

        for word in input.split_whitespace() {
            tree.insert(String::from(word), 0);
        }

        println!("The elements of the tree are -\n");
        println!("{:?}", tree.keys());
    }

    // test for 6.
    #[test]
    fn test_heap() {
        let mut tree = lib::Tree::new();
        let input = String::from("There has to be a better way to do this");

        for word in input.split_whitespace() {
            tree.insert(String::from(word), 0);
        }

        let _nodes = tree.clone();
        let _nodes = _nodes.iter();
        let mut nodes = vec![];

        for node in _nodes {
            nodes.push(node);
        }

        if nodes.len() == 2 {
            if nodes[1].0 < nodes[0].0 {
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
    }

    // test for 7.
    #[test]
    fn test_anagrams() {
        let mut tree = lib::Tree::new();
        let input = String::from("There has to be a better way to do this");

        for word in input.split_whitespace() {
            tree.insert(String::from(word), 0);
        }

        let words = tree.clone();

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
            }
        }
    }
}
