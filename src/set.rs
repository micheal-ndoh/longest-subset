pub struct Subset {
    nums: Vec<i32>,
    result: Vec<Vec<i32>>,
    current: Vec<i32>,
}

impl Subset {
    pub fn new(mut nums: Vec<i32>) -> Self {
        nums.sort(); 
        Subset {
            nums,
            result: vec![],
            current: vec![],
        } 
        
    }

    pub fn subsets_dup(&mut self) -> Vec<Vec<i32>> {
        self.backtrack(0);
        std::mem::take(&mut self.result)
    }

    fn backtrack(&mut self, start: usize) {
        self.result.push(self.current.clone()); // Add current subset to the result
        for i in start..self.nums.len() {
            if i > start && self.nums[i] == self.nums[i - 1] {
                continue; // Skip duplicates
            }
            self.current.push(self.nums[i]);
            self.backtrack(i + 1); // Recur for next elements
            self.current.pop(); // Backtrack
        }
    }
}

