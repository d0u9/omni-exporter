use std::collections::VecDeque;

use crate::error::Error;

use super::Stash;

#[derive(Debug)]
struct MemQueue<T> {
    data: VecDeque<T>,
}

impl<T> Default for MemQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MemQueue<T> {
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    fn push_back(&mut self, item: T) {
        self.data.push_back(item);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct MemCacheInner<T> {
    size: usize,
    data: T,
}

pub struct MemCache<T: Stash + Sized> {
    mem_limit: usize,
    mem_used: usize,
    queue: MemQueue<MemCacheInner<T>>,
}

impl<T: Stash + Sized> Default for MemCache<T> {
    fn default() -> Self {
        Self::new(usize::MAX)
    }
}

impl<T: Stash + Sized> MemCache<T> {
    pub fn new(max_mem: usize) -> Self {
        Self {
            mem_limit: max_mem,
            mem_used: 0,
            queue: MemQueue::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn capacity(&self) -> usize {
        self.mem_limit - self.mem_used
    }

    pub fn can_enqueue(&self, item: &T) -> bool {
        self.mem_used + item.sizeof() <= self.mem_limit
    }

    // When the item is too large to fit in the stash, it is returned.
    pub fn enqueue(&mut self, item: T) -> Option<T> {
        if !self.can_enqueue(&item) {
            Some(item)
        } else {
            self.mem_used += item.sizeof();
            self.queue.push_back(MemCacheInner {
                size: item.sizeof(),
                data: item,
            });
            None
        }
    }

    // Enqueues an item by first removing older items until there is enough space.
    // Returns the items that were removed to make space.
    fn make_space(&mut self, size: usize, collect_overflow: bool) -> Vec<T> {
        let mut overflow = Vec::new();

        while self.mem_used + size > self.mem_limit {
            if let Some(v) = self.queue.pop_front() {
                self.mem_used -= v.size;
                if collect_overflow {
                    overflow.push(v.data);
                }
            } else {
                break;
            }
        }

        overflow
    }

    pub fn enqueue_overflow(&mut self, item: T) -> Result<Vec<T>, (Error, T)> {
        let size = item.sizeof();
        let overflow = self.make_space(size, true);

        self.enqueue(item).map_or(Ok(overflow), |item| {
            Err((
                Error::NotEnoughSpace(format!("Not enough space to enqueue item: {} bytes", size)),
                item,
            ))
        })
    }

    pub fn enqueue_drop(&mut self, item: T) -> Result<(), (Error, T)> {
        let size = item.sizeof();
        self.make_space(size, false);

        self.enqueue(item).map_or(Ok(()), |item| {
            Err((
                Error::NotEnoughSpace(format!("Not enough space to enqueue item: {} bytes", size)),
                item,
            ))
        })
    }

    // When the stash is empty, None is returned.
    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front().map(|v| v.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Stash for u8 {
        fn sizeof(&self) -> usize {
            std::mem::size_of::<u8>()
        }
    }

    #[test]
    fn test_stash() {
        let mut stash: MemCache<u8> = MemCache::new(1024);
        assert_eq!(stash.mem_limit, 1024);
        assert_eq!(stash.mem_used, 0);
        assert_eq!(stash.queue.len(), 0);

        stash.enqueue(1);
        assert_eq!(stash.mem_used, 1);
        assert_eq!(stash.queue.len(), 1);
    }

    #[test]
    fn test_enqueue_overflow() {
        let mut stash: MemCache<u8> = MemCache::new(2);
        let overflow = stash.enqueue_overflow(1);
        assert_eq!(overflow.unwrap().len(), 0);
        let overflow = stash.enqueue_overflow(2);
        assert_eq!(overflow.unwrap().len(), 0);
        let overflow = stash.enqueue_overflow(3);
        assert_eq!(overflow.unwrap().len(), 1);
    }
}

#[cfg(test)]
mod tests_big_struct {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct MyBigStruct {
        id: u64,
        _space: [u8; 16],
    }

    impl Stash for MyBigStruct {
        fn sizeof(&self) -> usize {
            std::mem::size_of::<MyBigStruct>()
        }
    }

    #[test]
    fn test_enqueue_big_struct() {
        let mut stash: MemCache<MyBigStruct> = MemCache::new(2);
        let item = MyBigStruct {
            id: 1,
            _space: [0; 16],
        };
        stash.enqueue(item.clone());
        stash.enqueue(item.clone());

        let item = MyBigStruct {
            id: 99,
            _space: [0; 16],
        };
        let overflow = stash.enqueue(item);
        assert!(overflow.unwrap().id == 99);
    }

    #[test]
    fn test_enqueue_overflow_big_struct() {
        let mut stash: MemCache<MyBigStruct> = MemCache::new(2);
        let overflow = stash.enqueue_overflow(MyBigStruct {
            id: 1,
            _space: [0; 16],
        });
        dbg!(&overflow);
        assert!(overflow.is_err());
    }
}
