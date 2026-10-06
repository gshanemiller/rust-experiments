#[derive(Clone, Copy, PartialEq)]
enum DAGState {
  Unvisited,
  Visiting,
  Visited,
}

struct Graph {
  d_taskId:   Vec<u64>,
  d_taskDep:  Vec<Vec<u64>>,
  d_state:    Vec<DAGState>,
}

impl Graph {
  pub fn new(capacity: usize) -> Self {
    debug_assert!(capacity>0);
    Self {
      d_taskId: Vec::with_capacity(capacity),
      d_taskDep: Vec::with_capacity(capacity),
      d_state: Vec::with_capacity(capacity),
    }
  }

  pub fn addTask(&mut self, taskId: u64, deps: Vec<u64>) {
    debug_assert!(!self.d_taskId.contains(&taskId));
    self.d_taskId.push(taskId);
    self.d_taskDep.push(deps);
    self.d_state.push(DAGState::Unvisited);
  }

  fn indexForTaskId(&self, taskId: u64) -> usize {
    debug_assert!(self.d_taskId.contains(&taskId));
    if let Some(index) = self.d_taskId.iter().position(|&i| i == taskId) {
      return index;
    } else {
      panic!("Graph does not contain taskId {}\n", taskId);
    }
  }

  fn isCyclicHelper(&mut self, taskId: u64) -> bool {
    let outerIndex = self.indexForTaskId(taskId);
    self.d_state[outerIndex] = DAGState::Visiting;
    for i in 0..self.d_taskDep[outerIndex].len() {
      let depId = self.d_taskDep[outerIndex][i];
      let innerIndex = self.indexForTaskId(depId);
      match self.d_state[innerIndex] {
        DAGState::Unvisited => {
          if self.isCyclicHelper(depId) {
            return true;
          }
        }
        DAGState::Visited => {},
        DAGState::Visiting => return true,
      }
    }

    self.d_state[outerIndex] = DAGState::Visited;
    return false;
  }

  pub fn isCyclic(&mut self) -> bool {
    for i in 0..self.d_taskId.len() {
      if self.d_state[i] == DAGState::Unvisited {
        if self.isCyclicHelper(self.d_taskId[i]) {
          return true;
        }
      }
    }
    return false; 
  }
}

fn main() {
}

#[cfg(test)]                                                                                                            
mod tests {                                                                                                             
  use super::*;

  #[test]                                                                                                               
  fn emptyGraph() {                                                                                                       
    let mut g = Graph::new(1);
    assert!(!g.isCyclic());
  }

  #[test]                                                                                                               
  fn oneTaskGraph() {                                                                                                       
    let mut g = Graph::new(1);
    g.addTask(1, vec![]);
    assert!(!g.isCyclic());
  }

  #[test]                                                                                                               
  fn selfDepGraph() {                                                                                                       
    let mut g = Graph::new(1);
    g.addTask(1, vec![1]);
    assert!(g.isCyclic());
  }

  #[test]                                                                                                               
  fn simpleCycleGraph() {                                                                                                       
    let mut g = Graph::new(2);
    g.addTask(1, vec![2]);
    g.addTask(2, vec![1]);
    assert!(g.isCyclic());
  }

  #[test]                                                                                                               
  fn simpleNoCycleGraph() {                                                                                                       
    let mut g = Graph::new(3);
    g.addTask(1, vec![]);
    g.addTask(2, vec![1]);
    g.addTask(3, vec![2]);
    assert!(!g.isCyclic());
  }

  #[test]                                                                                                               
  fn simpleNoCycleGraph1() {                                                                                                       
    let mut g = Graph::new(3);
    g.addTask(1, vec![]);
    g.addTask(2, vec![1]);
    g.addTask(3, vec![1]);
    assert!(!g.isCyclic());
  }

  #[test]                                                                                                               
  fn mediumNoCycleGraph() {                                                                                                       
    let mut g = Graph::new(8);
    g.addTask(1, vec![]);
    g.addTask(2, vec![1]);
    g.addTask(3, vec![]);
    g.addTask(4, vec![3]);
    g.addTask(5, vec![3]);
    g.addTask(6, vec![]);
    g.addTask(7, vec![2]);
    g.addTask(8, vec![]);
    assert!(!g.isCyclic());
  }

  #[test]                                                                                                               
  fn mediumCycleGraph() {                                                                                                       
    let mut g = Graph::new(3);
    g.addTask(1, vec![3]);
    g.addTask(2, vec![1]);
    g.addTask(3, vec![2]);
    assert!(g.isCyclic());
  }
}
