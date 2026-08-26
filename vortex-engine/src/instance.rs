// -- leaked by @azixi0 on github
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::protocol::{ClassNameData, InstanceData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceGraphError {
    DuplicateId(u64),
    MissingParent { child: u64, parent: u64 },
    ParentCycle(u64),
}

#[derive(Debug, Default)]
pub struct InstanceGraph {
    instances: BTreeMap<u64, InstanceData>,
    children: BTreeMap<u64, Vec<u64>>,
}

impl InstanceGraph {
    pub fn build(instances: impl IntoIterator<Item = InstanceData>) -> Result<Self, InstanceGraphError> {
        let mut graph = Self::default();
        for instance in instances {
            let id = instance.id;
            if graph.instances.insert(id, instance).is_some() {
                return Err(InstanceGraphError::DuplicateId(id));
            }
        }
        for (&id, instance) in &graph.instances {
            if let Some(parent) = instance.parent {
                if !graph.instances.contains_key(&parent) {
                    return Err(InstanceGraphError::MissingParent { child: id, parent });
                }
                graph.children.entry(parent).or_default().push(id);
            }
        }
        for &id in graph.instances.keys() {
            let mut seen = BTreeSet::new();
            let mut cursor = Some(id);
            while let Some(current) = cursor {
                if !seen.insert(current) { return Err(InstanceGraphError::ParentCycle(id)); }
                cursor = graph.instances[&current].parent;
            }
        }
        Ok(graph)
    }

    pub fn get(&self, id: u64) -> Option<&InstanceData> { self.instances.get(&id) }

    pub fn get_children(&self, id: u64) -> Vec<&InstanceData> {
        self.children.get(&id).into_iter().flatten()
            .filter_map(|child| self.instances.get(child)).collect()
    }

    pub fn get_descendants(&self, id: u64) -> Vec<&InstanceData> {
        let mut output = Vec::new();
        let mut queue = VecDeque::from([id]);
        while let Some(parent) = queue.pop_front() {
            for child in self.children.get(&parent).into_iter().flatten() {
                if let Some(instance) = self.instances.get(child) {
                    output.push(instance);
                    queue.push_back(*child);
                }
            }
        }
        output
    }

    pub fn find_first_child_of_class(&self, id: u64, class_name: ClassNameData) -> Option<&InstanceData> {
        self.get_children(id).into_iter().find(|instance| instance.class_name == class_name)
    }

    pub fn destroy_subtree(&mut self, id: u64) -> Vec<InstanceData> {
        let mut ids = self.get_descendants(id).into_iter().map(|item| item.id).collect::<Vec<_>>();
        ids.push(id);
        if let Some(parent) = self.instances.get(&id).and_then(|item| item.parent) {
            if let Some(children) = self.children.get_mut(&parent) { children.retain(|child| *child != id); }
        }
        ids.into_iter().rev().filter_map(|item| {
            self.children.remove(&item);
            self.instances.remove(&item)
        }).collect()
    }
}
