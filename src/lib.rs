pub struct Heap {
    pub array: Vec<i64>,
    pub size: usize,
    pub capacity: usize,
}

impl Heap {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity != 0);
        let array = vec![0_i64; capacity];

        Heap {
            array,
            size: 0,
            capacity,
        }
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        let temp = self.array[a];
        self.array[a] = self.array[b];
        self.array[b] = temp;
    }

    //  If the left child exists and is greater than the current largest node, mark it as largest.
    /// Check if the right child exists.
    // If the right child exists and is greater than the current largest node, mark it as largest
    // If the largest node is not the root node, swap the root node with the largest node using the swap function.
    // Apply heapify operation to the affected subtree.
    pub fn heapify(&mut self, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < self.size && self.array[left] > self.array[largest] {
            largest = left;
        }

        if right < self.size && self.array[right] > self.array[largest] {
            largest = right;
        }

        if largest != i {
            self.swap(i, largest);
            self.heapify(largest);
        }
    }

    // Get the number of elements in the heap.
    // Identify the starting point for heapification. The function identifies the last non-leaf node in the heap, which is the parent of the last element. This is calculated as (n - 1) / 2.
    // The function then enters a loop that starts from the last non-leaf node and goes up to the root node.
    // Inside the loop, it calls heapify(heap, i) to ensure that the subtree rooted at i is a max heap heapifying all the levels.
    pub fn build_heap(&mut self) {
        let mut i = self.size;
        while i != 0 {
            self.heapify(i);
            i -= 1;
        }
    }

    pub fn increase_key(&mut self, index: usize, new_value: i64) {
        assert!(index > self.size);
        self.array[index] = new_value;
        let mut index = index;

        while index != 0 && self.array[(index - 1) / 2] < self.array[index] {
            self.swap(index, (index - 1) / 2);
            index = (index - 1) / 2;
        }
    }

    pub fn insert(&mut self, value: i64) -> Result<(), String> {
        if self.size == self.capacity {
            return Err("Heap Overflow!".to_string());
        }
        self.size += 1;
        let mut i = self.size - 1;
        self.array[i] = value;

        // Fix the heap property if it is violated
        while i != 0 && self.array[(i - 1) / 2] < self.array[i] {
            self.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
        Ok(())
    }

    pub fn extract_max(&mut self) -> i64 {
        if self.size <= 0 {
            return i64::MIN;
        }

        if self.size == 1 {
            self.size -= 1;
            return self.array[0];
        }

        // remove maximum value
        let root = self.array[0];
        self.array[0] = self.array[self.size - 1];
        self.size -= 1;
        self.heapify(0);
        root
    }

    pub fn print_heap(&self) {
        println!("{:?}", self.array);
    }

    pub fn remove_key(&mut self, index: usize) {
        assert!(index < self.size);

        // if index is last element, reduce the size
        if index == self.size {
            self.size -= 1;
            return;
        }

        //move the last item to index
        self.array[index] = self.array[self.size - 1];

        self.size -= 1;
        self.heapify(index);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn t_insert() {
        let mut h = Heap::new(3);
        h.insert(10).unwrap();
        assert_eq!(h.size, 1);
        h.insert(20).unwrap();
        assert_eq!(h.size, 2);
        h.insert(30).unwrap();
        assert_eq!(h.size, 3);

        assert!(h.insert(40).is_err());
    }

    #[test]
    fn t_remove_key() {
        let mut h = Heap::new(2);
        h.insert(10).unwrap();
        h.insert(20).unwrap();
        assert_eq!(h.size, 2);

        h.remove_key(1);
        assert_eq!(h.size, 1);
    }

    #[test]
    fn t_extract_max() {
        let mut h = Heap::new(2);
        h.insert(10).unwrap();
        let max = h.extract_max();
        assert_eq!(max, 10);
        h.insert(20).unwrap();

        let max = h.extract_max();
        assert_eq!(max, 20);
    }
}
