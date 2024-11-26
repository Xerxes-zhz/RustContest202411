use std::collections::HashMap;
#[derive(Debug)]
pub struct DSU {
    parent: HashMap<String, String>,
    rank: HashMap<String, usize>,
}
impl DSU {
    pub fn new() -> Self {
        DSU {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }
    pub fn init(&mut self, root: String, sub: String) {
        self.add(root.clone());
        self.add(sub.clone());
        self.union(root, sub);
    }
    fn add(&mut self, node: String) {
        if !self.parent.contains_key(&node) {
            self.parent.insert(node.clone(), node.clone());
            self.rank.insert(node, 0);
        }
    }
    fn find(&mut self, node: &str) -> String {
        if node != self.parent[node] {
            let root = self.find(self.parent[node].clone().as_str());
            self.parent.insert(node.to_string(), root.clone());
            root
        } else {
            node.to_string()
        }
    }
    pub fn union(&mut self, x: String, y: String) {
        let root_x = self.find(&x);
        let root_y = self.find(&y);

        if root_x != root_y {
            let rank_x = *self.rank.get(&root_x).unwrap_or(&0);
            let rank_y = *self.rank.get(&root_y).unwrap_or(&0);

            if rank_x > rank_y {
                self.parent.insert(root_y, root_x);
            } else if rank_x < rank_y {
                self.parent.insert(root_x, root_y);
            } else {
                self.parent.insert(root_y, root_x.clone());
                *self.rank.get_mut(&root_x).unwrap() += 1;
            }
        }
    }
    pub fn count_trees(&mut self) -> String {
        self.parent
            .iter()
            .filter(|(k, v)| k == v)
            .count()
            .to_string()
    }
}
