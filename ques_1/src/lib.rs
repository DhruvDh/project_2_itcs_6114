/**
 * Most of the code here is used to enable the generic-ness of this Tree, it can store any data, not just Strings. I wanted to learn how to work
 * with generic types and hence the code is not for just string.
 *
 * The reason nodes are key-value pairs instead of a single string is the same, if someone wants to insert some data that has no order
 * or does not implement the ord trait into the tree, he can just have a key that is ordered, and insert the data as value.
 *
 * Also I figured I could use the value to store anagram counts, even though some students told me we don't need to use the tree to count anagrams.
 * I didn't want to lose some grade in case they were mistaken.
 *
 * So a lot at the end is used for making iterators out of the tree, and can be ignored. I don't actually need the tree to be iterable, but I was
 * learning how to write iterators too.
 *
 * Please note that the code for iteration is heavily inspired from a crate I found on the internet, and I don't entirely understand how it works
 * either.
 */
// used to implement traits for comparision, makes it possible for custom data types to be compared.
// Here I am using it to make the key, value pair of each node comparable.
use std::cmp::Ord;
// Ordering is also used to help compare. I'll write more about it when I get to that part of the code.
use std::cmp::Ordering;
// This is used to tell print statements how to print custom data types. Helps in printing values in a node.
use std::fmt::{self, Debug};
// used to implement the into iterator trait which helps making the iterator into a tree.println!
// this is what makes using .iter() possible.
use std::iter::{FromIterator, IntoIterator};
// marker that's used to write some phantom data. this marker is read when we're unsure what the type of data is inside a tree.
// this is required because rust doesn't let you write code where it isn't possible to infer or at least specify the type.
// life would've been so much easier had I not used rust lol
use std::marker;
// just some helper functions for working with pointers
use std::ptr;

// deriving traits
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// just an enum for color
enum Color {
    Red,
    Black,
}

// Struc for a node and some helper structs for key, value pairs
struct TreeNode<K: Ord, V> {
    color: Color,
    left: NodePtr<K, V>,
    right: NodePtr<K, V>,
    parent: NodePtr<K, V>,
    key: K,
    value: V,
}

// returning a key-value pair
impl<K: Ord, V> TreeNode<K, V> {
    fn pair(self) -> (K, V) {
        (self.key, self.value)
    }
}

// just telling rust how to print an node in the tree
impl<K, V> Debug for TreeNode<K, V>
where
    K: Ord + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.key)
    }
}

// actual struct for the tree
pub struct Tree<K: Ord, V> {
    root: NodePtr<K, V>,
    len: usize,
}

// recursively printing the tree
impl<K: Ord + Debug, V: Debug> Tree<K, V> {
    fn tree_print(&self, node: NodePtr<K, V>, direction: i32) {
        if node.is_null() {
            return;
        }
        if direction == 0 {
            unsafe {
                println!("\t{:?}' is the root node", (*node.0));
            }
        } else {
            unsafe {
                match direction {
                    -1 => println!("\t{:?} <-- {:?}'s left child ", (*node.0), *node.parent().0,),
                    _ => println!(
                        "\t{:?} --> {:?}'s right child ",
                        (*node.0),
                        *node.parent().0,
                    ),
                }
            }
        }
        self.tree_print(node.left(), -1);
        self.tree_print(node.right(), 1);
    }

    pub fn print_tree(&self) {
        if self.root.is_null() {
            println!("This is a empty tree");
            return;
        }
        println!("\n\n\t-----------------------------",);
        self.tree_print(self.root, 0);
        println!("\t-----------------------------");
    }
}

// all tree utilities are implemented here
impl<K: Ord, V> Tree<K, V> {
    // Creates an empty tree.
    pub fn new() -> Tree<K, V> {
        Tree {
            root: NodePtr::null(),
            len: 0,
        }
    }

    // Returns the len of the tree.
    pub fn len(&self) -> usize {
        self.len
    }

    // implementing left rotation
    unsafe fn left_rotate(&mut self, mut node: NodePtr<K, V>) {
        let mut temp = node.right();
        node.set_right(temp.left());

        if !temp.left().is_null() {
            temp.left().set_parent(node.clone());
        }

        temp.set_parent(node.parent());
        if node == self.root {
            self.root = temp.clone();
        } else if node == node.parent().left() {
            node.parent().set_left(temp.clone());
        } else {
            node.parent().set_right(temp.clone());
        }

        temp.set_left(node.clone());
        node.set_parent(temp.clone());
    }

    // implementing right rotation
    unsafe fn right_rotate(&mut self, mut node: NodePtr<K, V>) {
        let mut temp = node.left();
        node.set_left(temp.right());

        if !temp.right().is_null() {
            temp.right().set_parent(node.clone());
        }

        temp.set_parent(node.parent());
        if node == self.root {
            self.root = temp.clone();
        } else if node == node.parent().right() {
            node.parent().set_right(temp.clone());
        } else {
            node.parent().set_left(temp.clone());
        }

        temp.set_right(node.clone());
        node.set_parent(temp.clone());
    }

    // code to fix balance of tree after inserting
    unsafe fn insert_fixup(&mut self, mut node: NodePtr<K, V>) {
        let mut parent;
        let mut gparent;

        while node.parent().is_red_color() {
            parent = node.parent();
            gparent = parent.parent();
            if parent == gparent.left() {
                // Case 1
                let mut uncle = gparent.right();
                if !uncle.is_null() && uncle.is_red_color() {
                    uncle.set_black_color();
                    parent.set_black_color();
                    gparent.set_red_color();
                    node = gparent;
                    continue;
                }

                // Case 2
                if parent.right() == node {
                    self.left_rotate(parent);
                    let temp = parent;
                    parent = node;
                    node = temp;
                }

                // Case 3
                parent.set_black_color();
                gparent.set_red_color();
                self.right_rotate(gparent);
            } else {
                // Case 1
                let mut uncle = gparent.left();
                if !uncle.is_null() && uncle.is_red_color() {
                    uncle.set_black_color();
                    parent.set_black_color();
                    gparent.set_red_color();
                    node = gparent;
                    continue;
                }

                // Case 2
                if parent.left() == node {
                    self.right_rotate(parent);
                    let temp = parent;
                    parent = node;
                    node = temp;
                }

                // Case 3
                parent.set_black_color();
                gparent.set_red_color();
                self.left_rotate(gparent);
            }
        }
        self.root.set_black_color();
    }

    // insert a value into the tree
    pub fn insert(&mut self, k: K, v: V) {
        self.len += 1;
        let mut node = NodePtr::new(k, v);
        let mut y = NodePtr::null();
        let mut x = self.root;

        while !x.is_null() {
            y = x;
            match node.cmp(&&mut x) {
                Ordering::Less => {
                    x = x.left();
                }
                _ => {
                    x = x.right();
                }
            };
        }
        node.set_parent(y);

        if y.is_null() {
            self.root = node;
        } else {
            match node.cmp(&&mut y) {
                Ordering::Less => {
                    y.set_left(node);
                }
                _ => {
                    y.set_right(node);
                }
            };
        }

        node.set_red_color();
        unsafe {
            self.insert_fixup(node);
        }
    }

    // finding node with a given key
    fn find_node(&self, k: &K) -> NodePtr<K, V> {
        if self.root.is_null() {
            return NodePtr::null();
        }
        let mut temp = &self.root;
        unsafe {
            loop {
                let next = match k.cmp(&(*temp.0).key) {
                    Ordering::Less => &mut (*temp.0).left,
                    Ordering::Greater => &mut (*temp.0).right,
                    Ordering::Equal => return *temp,
                };
                if next.is_null() {
                    break;
                }
                temp = next;
            }
        }
        NodePtr::null()
    }

    // get last child
    fn first_child(&self) -> NodePtr<K, V> {
        if self.root.is_null() {
            NodePtr::null()
        } else {
            let mut temp = self.root;
            while !temp.left().is_null() {
                temp = temp.left();
            }
            return temp;
        }
    }

    // get last child
    fn last_child(&self) -> NodePtr<K, V> {
        if self.root.is_null() {
            NodePtr::null()
        } else {
            let mut temp = self.root;
            while !temp.right().is_null() {
                temp = temp.right();
            }
            return temp;
        }
    }

    // Empties the tree without freeing objects in it.
    fn fast_clear(&mut self) {
        self.root = NodePtr::null();
    }

    // exposed function for delettion
    pub fn remove(&mut self, k: &K) -> Option<V> {
        let node = self.find_node(k);
        if node.is_null() {
            return None;
        }
        unsafe { Some(self.delete(node).1) }
    }

    // fixing the tree balance after deleting a node
    unsafe fn delete_fixup(&mut self, mut node: NodePtr<K, V>, mut parent: NodePtr<K, V>) {
        let mut other;
        while node != self.root && node.is_black_color() {
            if parent.left() == node {
                other = parent.right();
                if other.is_red_color() {
                    other.set_black_color();
                    parent.set_red_color();
                    self.left_rotate(parent);
                    other = parent.right();
                }

                if other.left().is_black_color() && other.right().is_black_color() {
                    other.set_red_color();
                    node = parent;
                    parent = node.parent();
                } else {
                    if other.right().is_black_color() {
                        other.left().set_black_color();
                        other.set_red_color();
                        self.right_rotate(other);
                        other = parent.right();
                    }
                    other.set_color(parent.get_color());
                    parent.set_black_color();
                    other.right().set_black_color();
                    self.left_rotate(parent);
                    node = self.root;
                    break;
                }
            } else {
                other = parent.left();
                if other.is_red_color() {
                    other.set_black_color();
                    parent.set_red_color();
                    self.right_rotate(parent);
                    other = parent.left();
                }

                if other.left().is_black_color() && other.right().is_black_color() {
                    other.set_red_color();
                    node = parent;
                    parent = node.parent();
                } else {
                    if other.left().is_black_color() {
                        other.right().set_black_color();
                        other.set_red_color();
                        self.left_rotate(other);
                        other = parent.left();
                    }
                    other.set_color(parent.get_color());
                    parent.set_black_color();
                    other.left().set_black_color();
                    self.right_rotate(parent);
                    node = self.root;
                    break;
                }
            }
        }

        node.set_black_color();
    }

    // i really should not have used rust
    unsafe fn delete(&mut self, node: NodePtr<K, V>) -> (K, V) {
        let mut child;
        let mut parent;
        let color;

        self.len -= 1;
        if !node.left().is_null() && !node.right().is_null() {
            let mut replace = node.right().min_node();
            if node == self.root {
                self.root = replace;
            } else {
                if node.parent().left() == node {
                    node.parent().set_left(replace);
                } else {
                    node.parent().set_right(replace);
                }
            }

            child = replace.right();
            parent = replace.parent();
            color = replace.get_color();
            if parent == node {
                parent = replace;
            } else {
                if !child.is_null() {
                    child.set_parent(parent);
                }
                parent.set_left(child);
                replace.set_right(node.right());
                node.right().set_parent(replace);
            }

            replace.set_parent(node.parent());
            replace.set_color(node.get_color());
            replace.set_left(node.left());
            node.left().set_parent(replace);

            if color == Color::Black {
                self.delete_fixup(child, parent);
            }

            let obj = Box::from_raw(node.0);
            return obj.pair();
        }

        if !node.left().is_null() {
            child = node.left();
        } else {
            child = node.right();
        }

        parent = node.parent();
        color = node.get_color();
        if !child.is_null() {
            child.set_parent(parent);
        }

        if self.root == node {
            self.root = child
        } else {
            if parent.left() == node {
                parent.set_left(child);
            } else {
                parent.set_right(child);
            }
        }

        if color == Color::Black {
            self.delete_fixup(child, parent);
        }

        let obj = Box::from_raw(node.0);
        return obj.pair();
    }

    // Return the keys iter
    pub fn keys(&self) -> Keys<K, V> {
        Keys { inner: self.iter() }
    }

    // Return the value iter
    pub fn values(&self) -> Values<K, V> {
        Values { inner: self.iter() }
    }

    // Return the key and value iter
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            head: self.first_child(),
            tail: self.last_child(),
            len: self.len,
            _marker: marker::PhantomData,
        }
    }

    // Return the key and mut value iter
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            head: self.first_child(),
            tail: self.last_child(),
            len: self.len,
            _marker: marker::PhantomData, // phantom data that consumes no space, used for type inference
        }
    }

    // pub fn compute_anagrams(&mut self) {
    //     let words = self.clone();

    //     for node in self.iter_mut() {
    //         for word in words.keys() {
    //             let mut is_anagram = true;

    //             if word.len() == node.0.len() {
    //                 for _char in node.0.to_lowercase().chars() {
    //                     if word.to_lowercase().contains(_char) {
    //                         continue;
    //                     } else {
    //                         is_anagram = false;
    //                         break;
    //                     }
    //                 }
    //                 if is_anagram {
    //                     *node.1 = *node.1 + 1;
    //                 }
    //             }
    //         }
    //     }
    // }
}

// pointer structs for node
#[derive(Debug)]
struct NodePtr<K: Ord, V>(*mut TreeNode<K, V>);

// helper to implement clone trait. basically telling the rust how to clone a node
impl<K: Ord, V> Clone for NodePtr<K, V> {
    fn clone(&self) -> NodePtr<K, V> {
        NodePtr(self.0)
    }
}

impl<K: Ord, V> Copy for NodePtr<K, V> {}

// helper to implement the ord trait
// ord trait allows nodes to be compared like any other variable.
// like if nodeA < nodeB is now possible

// there's a list of impls that need to be defined to get ord trait working, since node is basically a key-value pair of any data type
// we basically need to define for a bunch of cases how to compare
// the impls below do that
impl<K: Ord, V> Ord for NodePtr<K, V> {
    fn cmp(&self, other: &NodePtr<K, V>) -> Ordering {
        unsafe { (*self.0).key.cmp(&(*other.0).key) }
    }
}

// partial ordering
impl<K: Ord, V> PartialOrd for NodePtr<K, V> {
    fn partial_cmp(&self, other: &NodePtr<K, V>) -> Option<Ordering> {
        unsafe { Some((*self.0).key.cmp(&(*other.0).key)) }
    }
}

// partial equality
impl<K: Ord, V> PartialEq for NodePtr<K, V> {
    fn eq(&self, other: &NodePtr<K, V>) -> bool {
        self.0 == other.0
    }
}

impl<K: Ord, V> Eq for NodePtr<K, V> {}

// helper utilites for a node
// names are self-explanatory
// not all of these are actually needed, some are just to help make iterators
impl<K: Ord, V> NodePtr<K, V> {
    fn new(k: K, v: V) -> NodePtr<K, V> {
        let node = TreeNode {
            color: Color::Black,
            left: NodePtr::null(),
            right: NodePtr::null(),
            parent: NodePtr::null(),
            key: k,
            value: v,
        };
        NodePtr(Box::into_raw(Box::new(node)))
    }

    fn set_color(&mut self, color: Color) {
        if self.is_null() {
            return;
        }
        unsafe {
            (*self.0).color = color;
        }
    }

    fn set_red_color(&mut self) {
        self.set_color(Color::Red);
    }

    fn set_black_color(&mut self) {
        self.set_color(Color::Black);
    }

    fn get_color(&self) -> Color {
        if self.is_null() {
            return Color::Black;
        }
        unsafe { (*self.0).color }
    }

    fn is_red_color(&self) -> bool {
        if self.is_null() {
            return false;
        }
        unsafe { (*self.0).color == Color::Red }
    }

    fn is_black_color(&self) -> bool {
        if self.is_null() {
            return true;
        }
        unsafe { (*self.0).color == Color::Black }
    }

    fn is_left_child(&self) -> bool {
        self.parent().left() == *self
    }

    fn min_node(self) -> NodePtr<K, V> {
        let mut temp = self.clone();
        while !temp.left().is_null() {
            temp = temp.left();
        }
        return temp;
    }

    fn next(self) -> NodePtr<K, V> {
        if !self.right().is_null() {
            self.right().min_node()
        } else {
            let mut temp = self;
            loop {
                if temp.parent().is_null() {
                    return NodePtr::null();
                }
                if temp.is_left_child() {
                    return temp.parent();
                }
                temp = temp.parent();
            }
        }
    }

    fn set_parent(&mut self, parent: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).parent = parent }
    }

    fn set_left(&mut self, left: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).left = left }
    }

    fn set_right(&mut self, right: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).right = right }
    }

    fn parent(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).parent.clone() }
    }

    fn left(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).left.clone() }
    }

    fn right(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        unsafe { (*self.0).right.clone() }
    }

    fn null() -> NodePtr<K, V> {
        NodePtr(ptr::null_mut())
    }

    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

// helper to implement the clone trait
// there's a list of functions and structs that need to be defined to get .clone() working, this is one of those
impl<K: Ord + Clone, V: Clone> NodePtr<K, V> {
    unsafe fn deep_clone(&self) -> NodePtr<K, V> {
        let mut node = NodePtr::new((*self.0).key.clone(), (*self.0).value.clone());
        if !self.left().is_null() {
            node.set_left(self.left().deep_clone());
            node.left().set_parent(node);
        }
        if !self.right().is_null() {
            node.set_right(self.right().deep_clone());
            node.right().set_parent(node);
        }
        node
    }
}

// If key and value are both types that impl clone, we can call clone to get a copy.
impl<K: Ord + Clone, V: Clone> Clone for Tree<K, V> {
    fn clone(&self) -> Tree<K, V> {
        unsafe {
            let mut new = Tree::new();
            new.root = self.root.deep_clone();
            new.len = self.len;
            new
        }
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for Tree<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Tree<K, V> {
        let mut tree = Tree::new();
        tree.extend(iter);
        tree
    }
}

// Tree into iter
impl<K: Ord, V> Extend<(K, V)> for Tree<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

// an iter for keys in the tree
pub struct Keys<'a, K: Ord + 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

// helper to implement clone trait
// there's a list of impls that need to be defined to get .clone() working, this is one of those
impl<'a, K: Ord, V> Clone for Keys<'a, K, V> {
    fn clone(&self) -> Keys<'a, K, V> {
        Keys {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, K: Ord + Debug, V> fmt::Debug for Keys<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

// an iter for keys in the tree
impl<'a, K: Ord, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<(&'a K)> {
        self.inner.next().map(|(k, _)| k)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

pub struct Values<'a, K: 'a + Ord, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: Ord, V> Clone for Values<'a, K, V> {
    fn clone(&self) -> Values<'a, K, V> {
        Values {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, K: Ord + Debug, V: Debug> fmt::Debug for Values<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K: Ord, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<(&'a V)> {
        self.inner.next().map(|(_, v)| v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// helper for iter mut
pub struct ValuesMut<'a, K: 'a + Ord, V: 'a> {
    inner: IterMut<'a, K, V>,
}

impl<'a, K: Ord, V> Clone for ValuesMut<'a, K, V> {
    fn clone(&self) -> ValuesMut<'a, K, V> {
        ValuesMut {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, K: Ord + Debug, V: Debug> fmt::Debug for ValuesMut<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

// used for getting mutable references to values.
// used for updating anagram counts
impl<'a, K: Ord, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<(&'a mut V)> {
        self.inner.next().map(|(_, v)| v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// helper to implement iter trait
// there's a list of functions and structs that need to be defined to get .iter() working, this is one of those
pub struct IntoIter<K: Ord, V> {
    head: NodePtr<K, V>,
    len: usize,
}

// Drop all owned pointers if the collection is dropped
impl<K: Ord, V> Drop for IntoIter<K, V> {
    fn drop(&mut self) {
        for (_, _) in self {}
    }
}

// helper to implement iter trait
// there's a list of functions and structs that need to be defined to get .iter() working, this is one of those
impl<K: Ord, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        if self.len == 0 {
            return None;
        }

        if self.head.is_null() {
            return None;
        }

        let next = self.head.next();
        let obj = unsafe { Box::from_raw(self.head.0) };
        let (k, v) = obj.pair();
        self.head = next;
        self.len -= 1;
        Some((k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

// returns pointers to nodes in order.
pub struct Iter<'a, K: Ord + 'a, V: 'a> {
    head: NodePtr<K, V>,
    tail: NodePtr<K, V>,
    len: usize,
    _marker: marker::PhantomData<&'a ()>,
}

// helper to implement iter trait
// there's a list of functions and structs that need to be defined to get .iter() working, this is one of those
impl<'a, K: Ord + 'a, V: 'a> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Iter<'a, K, V> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: self._marker,
        }
    }
}

// helper to implement iter trait
// there's a list of functions and structs that need to be defined to get .iter() working, this is one of those
impl<'a, K: Ord + 'a, V: 'a> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        if self.len == 0 {
            return None;
        }

        if self.head.is_null() {
            return None;
        }

        let (k, v) = unsafe { (&(*self.head.0).key, &(*self.head.0).value) };
        self.head = self.head.next();
        self.len -= 1;
        Some((k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

// just like an iter, except it returns mutable references to nodes in the tree
// used when counting anagrams
pub struct IterMut<'a, K: Ord + 'a, V: 'a> {
    head: NodePtr<K, V>,
    tail: NodePtr<K, V>,
    len: usize,
    _marker: marker::PhantomData<&'a ()>,
}

// helper to implement iter trait
// there's a list of functions and structs that need to be defined to get .iter() working, this is one of those
impl<'a, K: Ord + 'a, V: 'a> Clone for IterMut<'a, K, V> {
    fn clone(&self) -> IterMut<'a, K, V> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: self._marker,
        }
    }
}

// helper to implement iter trait
// there's a list of functions and structs that need to be defined to get .iter() working, this is one of those
impl<'a, K: Ord + 'a, V: 'a> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        if self.len == 0 {
            return None;
        }

        if self.head.is_null() {
            return None;
        }

        let (k, v) = unsafe { (&(*self.head.0).key, &mut (*self.head.0).value) };
        self.head = self.head.next();
        self.len -= 1;
        Some((k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

// helper to implement iter trait
// there's a list of functions and structs that need to be defined to get .iter() working, this is one of those
impl<K: Ord, V> IntoIterator for Tree<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(mut self) -> IntoIter<K, V> {
        let iter = if self.root.is_null() {
            IntoIter {
                head: NodePtr::null(),
                len: self.len,
            }
        } else {
            IntoIter {
                head: self.first_child(),
                len: self.len,
            }
        };
        self.fast_clear();
        iter
    }
}
