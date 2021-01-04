use std::collections::VecDeque;

struct Log {
    cur_idx: usize,
    size: usize,
    log: VecDeque<usize>,
}

impl Log {
    fn new(size: usize) -> Self {
        Log {
            cur_idx: 0,
            size,
            log: VecDeque::new(),
        }
    }

    fn record(&mut self, order_id: usize) {
        self.log.push_back( order_id);
        if (self.cur_idx + 1 > self.size) {
            self.log.pop_front();
        } else {
            self.cur_idx = self.cur_idx + 1;
        }
    }

    fn get_last(self) -> Option<&usize> {
        self.log.get(self.cur_idx)
    }
}

fn main() {

}
