use std::collections::HashMap;

pub mod input;

pub type NodeId = usize;

pub struct FileSystem<'i> {
    nodes: HashMap<NodeId, Node<'i>>,
}

pub enum Node<'i> {
    File {
        id: NodeId,
        name: &'i str,
        size: usize,
    },
    Directory {
        id: NodeId,
        name: &'i str,
        children: Vec<NodeId>,
    },
}

impl<'i> From<&'i str> for FileSystem<'i> {
    fn from(input: &'i str) -> Self {
        let mut last_id = 0;
        let mut file_system = FileSystem {
            nodes: HashMap::new(),
        };
        file_system.nodes.insert(
            0,
            Node::Directory {
                id: 0,
                name: "/",
                children: vec![],
            },
        );
        let mut path = vec![0];
        let mut lines = input.lines().peekable();

        while let Some(line) = lines.next() {
            match &line[2..4] {
                "cd" => match &line[5..] {
                    "/" => {
                        path.truncate(1);
                    }
                    ".." => {
                        path.pop();
                    }
                    child_name => {
                        let current = file_system.nodes.get(path.last().unwrap()).unwrap();
                        if let Node::Directory { children, .. } = current {
                            path.push(
                                *children
                                    .iter()
                                    .find(|child_id| {
                                        let child = file_system.nodes.get(child_id).unwrap();
                                        if let Node::Directory { name, .. } = child {
                                            name == &child_name
                                        } else {
                                            false
                                        }
                                    })
                                    .unwrap(),
                            );
                        } else {
                            unreachable!("Parent should be a directory!");
                        }
                    }
                },
                "ls" => {
                    while lines.peek().is_some()
                        && lines.peek().unwrap().starts_with('$')
                    {
                        let insert_node;
                        if let Node::Directory { children, .. } =
                            file_system.nodes.get_mut(path.last().unwrap()).unwrap()
                        {
                            let (a, name) = lines.next().unwrap().split_once(' ').unwrap();
                            last_id += 1;
                            if a == "dir" {
                                insert_node = Node::Directory {
                                    id: last_id,
                                    name,
                                    children: vec![],
                                };
                            } else {
                                insert_node = Node::File {
                                    id: last_id,
                                    name,
                                    size: a.parse().unwrap(),
                                };
                            }
                            children.push(last_id);
                        } else {
                            unreachable!("Parent should be a directory!");
                        }
                        file_system.nodes.insert(last_id, insert_node);
                    }
                }
                _ => panic!("Wrong command!"),
            }
        }

        file_system
    }
}

impl<'i> FileSystem<'i> {
    pub fn root(&'i self) -> &'i Node {
        self.nodes.get(&0).unwrap()
    }

    pub fn iter(&'i self) -> NodeIterator<'i> {
        self.root().iter(self)
    }
}

impl<'i> Node<'i> {
    pub fn is_file(&self) -> bool {
        matches!(self, Node::File { .. })
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, Node::Directory { .. })
    }

    pub fn iter(&'i self, file_system: &'i FileSystem) -> NodeIterator<'i> {
        NodeIterator::new(file_system, self.id())
    }

    pub fn id(&self) -> usize {
        match self {
            Node::File { id, .. } => *id,
            Node::Directory { id, .. } => *id,
        }
    }

    pub fn name(&'i self) -> &'i str {
        match self {
            Node::File { name, .. } => name,
            Node::Directory { name, .. } => name,
        }
    }

    pub fn size(&self, file_system: &FileSystem) -> usize {
        match self {
            Node::File { size, .. } => *size,
            Node::Directory { children, .. } => children
                .iter()
                .map(|id| file_system.nodes.get(id).unwrap().size(file_system))
                .sum(),
        }
    }
}

pub struct NodeIterator<'i> {
    file_system: &'i FileSystem<'i>,
    node_id: NodeId,
    self_done: bool,
    child_iters: Vec<Self>,
}

impl<'i> NodeIterator<'i> {
    pub fn new(file_system: &'i FileSystem, node_id: NodeId) -> Self {
        Self {
            file_system,
            node_id,
            self_done: false,
            child_iters: match file_system.nodes.get(&node_id).unwrap() {
                Node::File { .. } => vec![],
                Node::Directory { children, .. } => children
                    .iter()
                    .map(|child| NodeIterator::new(file_system, *child))
                    .collect(),
            },
        }
    }
}

impl<'i> Iterator for NodeIterator<'i> {
    type Item = &'i Node<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.self_done {
            self.self_done = true;
            Some(self.file_system.nodes.get(&self.node_id).unwrap())
        } else if let Some(child_iter) = self.child_iters.last_mut() {
            if let Some(node) = child_iter.next() {
                Some(node)
            } else {
                self.child_iters.pop();
                self.next()
            }
        } else {
            None
        }
    }
}
