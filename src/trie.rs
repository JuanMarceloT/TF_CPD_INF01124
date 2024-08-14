
trait Identifiable {
    fn id(&self) -> i32;
}

impl Identifiable for i32 {
    fn id(&self) -> i32 {
        *self
    }
}

impl Identifiable for char {
    fn id(&self) -> i32 {
        let r = (*self) as i32;
        r
    }
}
#[derive(Default, Debug, Clone)]
struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    modulo: usize,
}

impl<K:Eq + Identifiable, V> HashMap <K, V> {
    fn new(modulo: usize) -> Self {
        let mut buckets = Vec::with_capacity(modulo);
        for _ in 0..modulo {
            buckets.push(Vec::new());
        }
        HashMap { buckets, modulo }
    }

    fn hash(&self, key: &K) -> usize {
        (key.id() as usize) % self.modulo
    }

    fn insert(&mut self, key: K, value: V) {
        let bucket = self.hash(&key);
        self.buckets[bucket].push((key, value));
    }

    fn get(&mut self, key: &K) -> Option<&mut V>
    where
        K: Eq,
    {
        let bucket_index = self.hash(key);
        for &mut (ref existing_key, ref mut existing_value) in &mut self.buckets[bucket_index] {
            if (*existing_key) == (*key) {
                return Some(existing_value);
            }
        }
        None
    }

}



#[derive(Default, Debug, Clone)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
    has_id: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(26),
            is_end_of_word: false,
            has_id: false
        }
    }
}

struct Trie {
    root: TrieNode,
}
#[allow(dead_code)]
impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert_with_id(&mut self, word: &str, id: u32) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            if node.children.get(&ch).is_none() {
                node.children.insert(ch, TrieNode::new());
            }
            node = node.children.get(&ch).unwrap();
        }
        node.is_end_of_word = true;
        if let Some(c) = char::from_u32(id) {
            node.children.insert(c, TrieNode::new());
            node = node.children.get(&c).unwrap();
            node.has_id = true;
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            if node.children.get(&ch).is_none() {
                node.children.insert(ch, TrieNode::new());
            }
            node = node.children.get(&ch).unwrap();
        }
        node.is_end_of_word = true;
    }
    fn search(&mut self, word: &str) -> bool {
        let mut node = &mut self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }
        //println!("{:?}", node.children);
        node.is_end_of_word
    }

    fn get_id(&mut self, word: &str) -> Option<i32> {
        let mut node = &mut self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(next_node) => node = next_node,
                None => return None,
            }
        }

        
        if node.is_end_of_word {
            for item in node.children.buckets.iter() {
                if !item.is_empty() && item[0].1.has_id {
                    return Some(item[0].0 as i32);
                }
            }
        }
        
        None
    }


    fn starts_with(&mut self, prefix: &str) -> bool {
        let mut node = &mut self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }
        true
    }

    fn get_words_starting_with(&mut self, prefix: &str) -> Vec<String> {
        let mut node = self.root.clone();
            for ch in prefix.chars() {
                match node.children.get(&ch) {
                    Some(next_node) => node = next_node.clone(),
                    None => return Vec::new(),
                }
        }
        
        let mut result = Vec::new();
        
        self.collect_words(&node, &prefix.to_string(), &mut result);
        result
    }

    fn collect_words(&mut self, node: &TrieNode, prefix: &String, result: &mut Vec<String>) {

        if node.is_end_of_word {
            result.push(prefix.clone());
        }

        for bucket in node.children.buckets.iter() {

            if let Some((character, child_node)) = bucket.first() {
                let mut new_prefix = prefix.clone();
                new_prefix.push(*character);
                self.collect_words(child_node, &new_prefix, result);
            }
            
         }
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_insertion_and_get() {
        let mut map = HashMap::new(10);
        
        // Test insertion
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(10, "ten");
        
        // Test retrieval
        assert_eq!(map.get(&1), Some(&mut "one"));
        assert_eq!(map.get(&2), Some(&mut "two"));
        assert_eq!(map.get(&10), Some(&mut "ten"));
        assert_eq!(map.get(&3), None); // Key does not exist
    }

    #[test]
    fn test_trie_insertion_and_search() {
        let mut trie = Trie::new();
        
        // Test insertion
        trie.insert("hello");
        trie.insert("hell");
        trie.insert("helicopter");
        
        // Test search
        assert!(trie.search("hello"));
        assert!(trie.search("hell"));
        assert!(trie.search("helicopter"));
        assert!(!trie.search("helic"));
        assert!(!trie.search("helloo"));
    }

    #[test]
    fn test_trie_insert_with_id_and_get_id() {
        let mut trie = Trie::new();
        
        // Test insertion with ID
        trie.insert_with_id("apple", 65);
        trie.insert_with_id("banana", 66);
        trie.insert("cherry");
        
        // Test get_id
        assert_eq!(trie.get_id("apple"), Some(65));
        assert_eq!(trie.get_id("banana"), Some(66));
        assert_eq!(trie.get_id("cherry"), None); // no id
    }

    #[test]
    fn test_trie_starts_with() {
        let mut trie = Trie::new();
        
        // Test insertion
        trie.insert("apple");
        trie.insert("app");
        trie.insert("banana");
        
        // Test starts_with
        assert!(trie.starts_with("app"));
        assert!(trie.starts_with("banana"));
        assert!(!trie.starts_with("bananas"));
        assert!(!trie.starts_with("bat"));
    }

    #[test]
    fn test_trie_get_words_starting_with() {
        let mut trie = Trie::new();
        
        // Test insertion
        trie.insert("apple");
        trie.insert("app");
        trie.insert("banana");
        trie.insert("bat");
        
        // Test get_words_starting_with
        let words_with_app = trie.get_words_starting_with("app");
        assert_eq!(words_with_app, vec!["app".to_string(), "apple".to_string()]);
        
        let words_with_b = trie.get_words_starting_with("b");
        assert_eq!(words_with_b, vec!["banana".to_string(), "bat".to_string()]);
        
        let words_with_nonexistent_prefix = trie.get_words_starting_with("nonexistent");
        assert_eq!(words_with_nonexistent_prefix, Vec::<String>::new());
    }
}
