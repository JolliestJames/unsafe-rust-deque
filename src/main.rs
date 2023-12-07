use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a mut T>,
}

pub struct IntoIter<T> {
    list: LinkedList<T>
}

pub struct CursorMut<'a, T> {
    list: &'a mut LinkedList<T>,
    cursor: Link<T>,
    index: Option<usize>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                self.back = Some(new);
            }

            self.front = Some(new);
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            if let Some(old) = self.back {
                (*old.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(old);
            } else {
                self.front = Some(new)
            }

            self.back = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                self.front = boxed_node.back;
                if let Some(new) = self.front {
                    (*new.as_ptr()).front = None;
                } else {
                    self.back = None;
                }

                self.len -= 1;
                result
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.back.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                self.back = boxed_node.front;
                if let Some(new) = self.back {
                    (*new.as_ptr()).back = None;
                } else {
                    self.front = None;
                }

                self.len -= 1;
                result
            })
        }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.front.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn front_mut(&mut self) -> Option<&T> {
        unsafe { self.front.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.back.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn back_mut(&self) -> Option<&T> {
        unsafe { self.back.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }

    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            list: self,
            cursor: None,
            index: None,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();

        for item in self {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state)
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.len = (*node.as_ptr()).back;
                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;
                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len =- 1;
                self.back = (*node.as_ptr()).front;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}

impl<'a, T> CursorMut<'a, T> {
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn move_next(&mut self) {
        if let Some(cursor) = self.cursor {
            unsafe {
                self.cursor = (*cursor.as_ptr()).back;
                if self.cursor.is_some() {
                    *self.index.as_mut().unwrap() += 1;
                } else {
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            self.cursor = self.list.front;
            self.index = Some(0);
        } else {}
    }

    pub fn move_prev(&mut self) {
        if let Some(cursor) = self.cursor {
            unsafe {
                self.cursor = (*cursor.as_ptr()).front;
                if self.cursor.is_some() {
                    *self.index.as_mut().unwrap() -= 1;
                } else {
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            self.cursor = self.list.back;
            self.index = Some(self.list.len - 1);
        } else {}
    }

    pub fn current(&mut self) -> Option<&mut T> {
        unsafe { self.cursor.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn peek_next(&mut self) -> Option<&mut T> {
        unsafe {
            let next = if let Some(cursor) = self.cursor {
                (*cursor.as_ptr()).back
            } else {
                self.list.front
            };

            next.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn peek_prev(&mut self) -> Option<&mut T> {
        unsafe {
            let prev = if let Some(cursor) = self.cursor {
                (*cursor.as_ptr()).front
            } else {
                self.list.back
            };

            prev.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn split_before(&mut self) -> LinkedList<T> {
        if let Some(cursor) = self.cursor {
            unsafe {
                let old_len = self.list.len;
                let old_index = self.index.unwrap();
                let prev = (*cursor.as_ptr()).front;

                let new_len = old_len - old_index;
                let new_front = self.cursor;
                let new_back = self.list.back;
                let new_index = Some(0);

                let output_len = old_len - new_len;
                let output_front = self.list.front;
                let output_back = prev;

                if let Some(prev) = prev {
                    (*cursor.as_ptr()).front = None;
                    (*prev.as_ptr()).back = None;
                }

                self.list.len = new_len;
                self.list.front = new_front;
                self.list.back = new_back;
                self.index = new_index;

                LinkedList {
                    front: output_front,
                    back: output_back,
                    len: output_len,
                    _boo: PhantomData,
                }
            }
        } else {
            std::mem::replace(self.list, LinkedList::new())
        }
    }

    pub fn split_after(&mut self) -> LinkedList<T> {
        if let Some(cursor) = self.cursor {
            unsafe {
                let old_len = self.list.len;
                let old_index = self.index.unwrap();
                let next = (*cursor.as_ptr()).back;

                let new_len = old_index + 1;
                let new_back = self.cursor;
                let new_front = self.list.front;
                let new_index = Some(old_index);

                let output_len = old_len - new_len;
                let output_back = self.list.back;
                let output_front = next;

                if let Some(prev) = next {
                    (*cursor.as_ptr()).back = None;
                    (*next.as_ptr()).front = None;
                }

                self.list.len = new_len;
                self.list.front = new_front;
                self.list.back = new_back;
                self.index = new_index;

                LinkedList {
                    front: output_front,
                    back: output_back,
                    len: output_len,
                    _boo: PhantomData,
                }
            }
        } else {
            std::mem::replace(self.list, LinkedList::new())
        }
    }

    pub fn splice_before(&mut self, mut input: LinkedList<T>) {
        unsafe {
            if input.is_empty() {
            } else if let Some(cursor) = self.cursor {
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                if let Some(prev) = (*cursor.as_ptr()).front {
                    (*prev.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(prev);
                    (*cursor.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(cursor);
                } else {
                    (*cursor.as_ptr()).back = Some(in_back);
                    (*in_back.as_ptr()).back = Some(cursor);
                    self.list.front = Some(in_front);
                }

                *self.index.as_mut().unwrap() += input.len;
            } else if let Some(back) = self.list.back {
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                (*back.as_ptr()).back = Some(in_front);
                (*in_front.as_ptr()).front = Some(back);
                self.list.back = Some(in_back);
            } else {
                std::mem::swap(self.list, &mut input);
            }

            self.list.len += input.len;
            input.len = 0;
        }
    }

    pub fn splice_after(&mut self, mut input: LinkedList<T>) {
        unsafe {
            if input.is_empty() {
            } else if let Some(cursor) = self.cursor {
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                if let Some(prev) = (*cursor.as_ptr()).front {
                    (*prev.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(prev);
                    (*cursor.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(cursor);
                } else {
                    (*cursor.as_ptr()).back = Some(in_back);
                    (*in_back.as_ptr()).back = Some(cursor);
                    self.list.front = Some(in_front);
                }

                *self.index.as_mut().unwrap() += input.len;
            } else if let Some(back) = self.list.back {
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                (*back.as_ptr()).back = Some(in_front);
                (*in_front.as_ptr()).front = Some(back);
                self.list.back = Some(in_back);
            } else {
                std::mem::swap(self.list, &mut input);
            }

            self.list.len += input.len;
            input.len = 0;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
