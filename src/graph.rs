
pub struct GraphBuilder {
    buff: String,
    name: String,
    curr: usize, 
    kind: GraphKind,
    shape: NodeShape
}

impl GraphBuilder {
    pub fn new(name: &str, kind: GraphKind, shape: NodeShape) -> Self {
        Self {
            buff: String::new(),
            name: name.to_owned(),
            curr: 0,
            kind: kind,
            shape: shape
        }
    } 

    pub fn add_node(&mut self, label: &str) -> usize {
        let txt = format!("\t{} [label=\"{}\", shape=\"{}\"]\n", self.curr, label, self.get_shape());
        self.buff.push_str(&txt);
        let out = self.curr;
        self.curr += 1;
        out
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(from < self.curr);
        assert!(to < self.curr);

        let tmp = format!("\t{} {} {}\n", from, self.get_edge(), to);
        self.buff.push_str(&tmp);
    }

    pub fn add_child(&mut self, label: &str, root: usize) -> usize {
        let output = self.add_node(label);
        self.add_edge(root, output);
        output
    }

    pub fn set_shape(&mut self, shape: NodeShape) {
        self.shape = shape;
    }

    pub fn export(&self) -> String {
        let tmp = match self.kind {
            GraphKind::Direct => "digraph",
            GraphKind::Undirect => "graph"
        };
        format!("{} \"{}\" {{\n{}\n}}",  tmp, self.name, self.buff)
    }

    fn get_edge(&self) -> &'static str {
        match self.kind {
            GraphKind::Direct => "->",
            GraphKind::Undirect => "--"
        }
    }
    
    fn get_shape(&self) -> &'static str {
        match self.shape {
            NodeShape::Diamond => "diamond",
            NodeShape::Oval => "oval",
            NodeShape::Rectangle => "rectangle"
        }
    }
}

pub enum GraphKind {
    Direct,
    Undirect
}

pub enum NodeShape {
    Rectangle,
    Oval,
    Diamond
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_graph() {
        let mut builder = GraphBuilder::new("test", GraphKind::Direct, NodeShape::Rectangle);
        assert_eq!(builder.add_node("zero"), 0);
        assert_eq!(builder.add_node("one"), 1);
        assert_eq!(builder.add_node("two"), 2);

        builder.add_edge(0, 1);
        builder.add_edge(1, 2);
        builder.add_edge(2, 0);

        let dot_code = builder.export();
        let ans = "digraph \"test\" {\n\
                  \t0 [label=\"zero\", shape=\"rectangle\"]\n\
                  \t1 [label=\"one\", shape=\"rectangle\"]\n\
                  \t2 [label=\"two\", shape=\"rectangle\"]\n\
                  \t0 -> 1\n\
                  \t1 -> 2\n\
                  \t2 -> 0\n\n\
                  }";
        assert_eq!(dot_code, ans);
    }


    #[should_panic]
    #[test]
    fn test_error_edge() {
        let mut builder = GraphBuilder::new("test", GraphKind::Direct, NodeShape::Rectangle);
        builder.add_node("zero");
        builder.add_node("one");

        builder.add_edge(0, 1);
        builder.add_edge(1, 2);
        builder.add_edge(2, 0);
    }

}
















