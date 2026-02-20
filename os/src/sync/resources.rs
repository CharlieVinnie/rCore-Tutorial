use alloc::vec::Vec;

/// Resource requirements for mutexes and semaphores
pub struct Resources {
    a: Vec<(usize, isize)>,
}

/// Resources available for mutexes and semaphores
pub struct ResourceBank {
    a: Vec<isize>,
}

impl Resources {
    /// Create a new empty resources
    pub fn empty() -> Self {
        Self {
            a: Vec::new(),
        }
    }

    /// Add a resource to the resources
    pub fn add(&mut self, id: usize, count: isize) {
        if let Some((idx, item)) = self.a.iter_mut().enumerate().find(|(_idx, (i, _c))| *i == id) {
            item.1 += count;
            assert!(item.1 >= 0);
            if item.1 == 0 {
                self.a.remove(idx);
            }
        } else {
            assert!(count >= 0);
            self.a.push((id, count));
        }
    }

    /// Merge all resources from `other` into self, leaving `other` empty
    pub fn merge(&mut self, other: &mut Resources) {
        for (id, count) in other.a.drain(..) {
            self.add(id, count);
        }
    }
}

impl ResourceBank {
    /// Create a new resource bank
    pub fn new(a: Vec<isize>) -> Self {
        Self {
            a,
        }
    }

    /// Add resources to the resource bank
    pub fn add(&mut self, resources: &Resources) {
        for &(id, count) in resources.a.iter() {
            self.a[id] += count;
        }
    }

    /// Check if the resource bank has enough resources for the given resources
    pub fn is_enough_for(&self, resources: &Resources) -> bool {
        resources.a.iter().all(|&(id, count)| self.a[id] >= count)
    }
}

/// Check if the resource bank has enough resources for the given resources
pub fn check_resource_deadlock(mut bank: ResourceBank, mut res_vec: Vec<(&Resources, &Resources)>) -> bool {
    let mut progressed = true;
    while progressed && !res_vec.is_empty() {
        progressed = false;
        res_vec.retain(|(alloc, request)| {
            if bank.is_enough_for(request) {
                bank.add(alloc);
                progressed = true;
                false
            } else {
                true
            }
        });
    }
    !res_vec.is_empty()
}