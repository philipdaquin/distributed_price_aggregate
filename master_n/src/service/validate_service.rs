use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct NodeSignatureValidator { 
    sig_map: HashMap<String, String>
}


impl NodeSignatureValidator { 
    pub fn new() -> Self { 
        return Self { 
            sig_map: HashMap::new()
        }
    }
    pub fn add_signature(&mut self, node_id: &str, sig: &str) { 
        self.sig_map.insert(node_id.to_string(), sig.to_string());
    }

    pub fn validate_sig(&self, node_id: String, sig: String) -> bool { 
        match self.sig_map.get(&node_id) {
            Some(stored_signature) => {
                if *stored_signature == sig { 
                    log::info!("✅✅ Validated Signature for: {node_id}");
                    return true
                } 
                log::warn!("❌❌ Invalid signature for : {node_id}");

                return false
            },
            None => {
                log::warn!("❌❌ Invalid signature for: {node_id}");
                return false
            },
        }
    }
}


pub fn initialise_valid_signatures() -> NodeSignatureValidator { 
    let mut node = NodeSignatureValidator::new();

    node.add_signature("worker_a", "worker_a");
    node.add_signature("worker_b", "worker_b");
    node.add_signature("worker_d", "worker_d");
    node.add_signature("worker_c", "worker_c");
    node.add_signature("worker_e", "worker_e");
    node.add_signature("worker_f", "worker_f");

    return node
}