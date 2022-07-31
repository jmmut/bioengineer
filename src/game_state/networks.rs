use crate::IVec3;
use crate::map::{CellIndex, TileType};

pub struct Networks {
    networks: Vec<Network>,
}

pub struct Network {
    pub nodes: Vec<Node>,
}

pub struct Node {
    pub position: CellIndex,
    pub tile: TileType,
}

impl Networks {
    pub fn new() -> Self {
        Networks { networks: Vec::new() }
    }

    pub fn add(&mut self, cell_index: CellIndex, new_machine: TileType) {
        if self.replace_if_present(cell_index, new_machine) {
            return;
        }
        let adjacent_networks = self.get_adjacent_networks(cell_index);
        if adjacent_networks.len() > 1 {
            todo!()
        } else {
            let node = Node { position: cell_index, tile: new_machine };
            if adjacent_networks.len() == 1 {
                let network_index = *adjacent_networks.first().unwrap();
                let option:Option<&mut Network> = self.networks.get_mut(network_index);
                option.unwrap().add(node);
            } else if adjacent_networks.len() == 0 {
                let mut network = Network::new();
                network.add(node);
                self.networks.push(network);
            }
        }
    }

    fn replace_if_present(&mut self, cell_index: CellIndex, new_machine: TileType) -> bool {
        for network in &mut self.networks {
            if let Option::Some(found) = network.get(cell_index) {
                *found = new_machine;
                return true;
            }
        }
        return false;
    }

    pub fn get_adjacent_networks(&self, cell_index: CellIndex) -> Vec<usize> {
        // todo()!
        if self.networks.len() > 0 {
            vec![0]
        } else {
            Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.networks.len()
    }
}

impl Network {
    pub fn new() -> Self {
        Network { nodes: Vec::new() }
    }

    pub fn get(&mut self, cell_index: CellIndex) -> Option<&mut TileType> {
        for node in &mut self.nodes {
            if node.position == cell_index {
                return Option::Some(&mut node.tile);
            }
        }
        Option::None
    }

    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{CellIndex, TileType};
    use super::*;

    #[test]
    fn test_join_networks() {
        let mut networks = Networks::new();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 2), TileType::MachineAssembler);
        assert_eq!(networks.len(), 2);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        assert_eq!(networks.len(), 1);
    }
    
    #[test]
    fn test_append_to_network() {
        let mut networks = Networks::new();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        assert_eq!(networks.len(), 1);
    }

    #[test]
    fn test_replace_machine() {
        let mut networks = Networks::new();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineDrill);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks.networks.get(0).unwrap().nodes.first().unwrap().tile,
                   TileType::MachineDrill);
    }
}
