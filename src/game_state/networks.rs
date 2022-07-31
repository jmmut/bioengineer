use crate::game_state::robots::CellIndexDiff;
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
        Networks {
            networks: Vec::new(),
        }
    }

    pub fn add(&mut self, cell_index: CellIndex, new_machine: TileType) {
        if self.replace_if_present(cell_index, new_machine) {
            return;
        }
        let node = Node {
            position: cell_index,
            tile: new_machine,
        };
        let mut adjacent_networks = self.get_adjacent_networks(cell_index);
        if adjacent_networks.len() > 1 {
            self.join_networks_and_add_node(&mut adjacent_networks, node);
        } else if adjacent_networks.len() == 1 {
            self.add_to_network(node, &mut adjacent_networks);
        } else if adjacent_networks.len() == 0 {
            self.add_new_network_with_node(node);
        }
    }

    fn replace_if_present(&mut self, cell_index: CellIndex, new_machine: TileType) -> bool {
        for network in &mut self.networks {
            if network.replace_if_present(cell_index, new_machine) {
                return true;
            }
        }
        return false;
    }

    pub fn get_adjacent_networks(&self, cell_index: CellIndex) -> Vec<usize> {
        let mut adjacents = Vec::new();
        let mut i = 0;
        for network in &self.networks {
            if network.is_adjacent(cell_index) {
                adjacents.push(i);
            }
            i += 1;
        }
        adjacents
    }

    fn join_networks_and_add_node(&mut self, adjacent_networks: &mut Vec<usize>, node: Node) {
        let (kept, to_be_merged) = adjacent_networks.split_first().unwrap();
        let mut networks_to_be_merged = Vec::new();
        for i in (to_be_merged.len() - 1)..=0 {
            networks_to_be_merged.push(self.networks.remove(i));
        }
        let network_kept = self.networks.get_mut(*kept).unwrap();
        while let Option::Some(last) = networks_to_be_merged.pop() {
            network_kept.join(last);
        }
        network_kept.add(node)
    }

    fn add_to_network(&mut self, node: Node, adjacent_networks: &mut Vec<usize>) {
        let network_index = *adjacent_networks.first().unwrap();
        let option: Option<&mut Network> = self.networks.get_mut(network_index);
        option.unwrap().add(node);
    }

    fn add_new_network_with_node(&mut self, node: Node) {
        let mut network = Network::new();
        network.add(node);
        self.networks.push(network);
    }

    pub fn len(&self) -> usize {
        self.networks.len()
    }
}

impl Network {
    pub fn new() -> Self {
        Network { nodes: Vec::new() }
    }

    #[allow(unused)]
    fn get(&mut self, cell_index: CellIndex) -> Option<&mut TileType> {
        for node in &mut self.nodes {
            if node.position == cell_index {
                return Option::Some(&mut node.tile);
            }
        }
        Option::None
    }

    fn replace_if_present(&mut self, cell_index: CellIndex, new_machine: TileType) -> bool {
        for node in &mut self.nodes {
            if node.position == cell_index {
                node.tile = new_machine;
                return true;
            }
        }
        return false;
    }

    pub fn is_adjacent(&self, cell_index: CellIndex) -> bool {
        for node in &self.nodes {
            let diff: CellIndexDiff = node.position - cell_index;
            if adjacent_positions().contains(&diff) {
                return true;
            }
        }
        return false;
    }

    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn join(&mut self, other: Network) {
        for node in other.nodes {
            self.add(node);
        }
    }
}

fn adjacent_positions() -> Vec<CellIndexDiff> {
    vec![
        CellIndexDiff::new(1, 0, 0),
        CellIndexDiff::new(-1, 0, 0),
        CellIndexDiff::new(0, 1, 0),
        CellIndexDiff::new(0, -1, 0),
        CellIndexDiff::new(0, 0, 1),
        CellIndexDiff::new(0, 0, -1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{CellIndex, TileType};

    #[test]
    fn test_join_networks() {
        let mut networks = Networks::new();
        networks.add(CellIndex::new(0, 0, 0), TileType::MachineAssembler);
        networks.add(CellIndex::new(0, 0, 2), TileType::MachineAssembler);
        assert_eq!(networks.len(), 2);
        networks.add(CellIndex::new(0, 0, 1), TileType::MachineAssembler);
        assert_eq!(networks.len(), 1);
        assert_eq!(networks.networks.get(0).unwrap().nodes.len(), 3);
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
        assert_eq!(
            networks
                .networks
                .get(0)
                .unwrap()
                .nodes
                .first()
                .unwrap()
                .tile,
            TileType::MachineDrill
        );
    }

    #[test]
    fn test_is_adjacent() {
        let mut network = Network::new();
        let adjacent = CellIndex::new(0, 0, 0);
        let position = CellIndex::new(0, 0, 1);
        assert_eq!(network.is_adjacent(adjacent), false);
        network.add(Node {
            tile: TileType::MachineDrill,
            position,
        });
        assert_eq!(network.is_adjacent(adjacent), true);
    }
}
