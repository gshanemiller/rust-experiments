use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

/// A single step in an interleaved sequence (which machine advanced, and to which state).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step<A, B> {
  A(A),
  B(B),
}

/// Minimal interface an FSM must provide.
pub trait Fsm {
  type State: Clone + Eq + Hash + Debug;

  fn initial(&self) -> Self::State;

  /// Returns every valid next state reachable in one step from `s`.
  fn transitions(&self, s: &Self::State) -> Vec<Self::State>;
}

/// Generate all interleavings of the two machines.
///
/// Each result is a sequence of `Step`s.  Because the underlying graphs are
/// acyclic the recursion is guaranteed to terminate.
pub fn interleave<A, B>(a: &A, b: &B) -> Vec<Vec<Step<A::State, B::State>>>
where
  A: Fsm,
  B: Fsm,
{
  let mut results = Vec::new();
  let mut path = Vec::new();
  let mut visited = HashSet::new();

  fn dfs<A, B>(
    a: &A,
    b: &B,
    sa: A::State,
    sb: B::State,
    path: &mut Vec<Step<A::State, B::State>>,
    results: &mut Vec<Vec<Step<A::State, B::State>>>,
    visited: &mut HashSet<(A::State, B::State)>,
  ) where
    A: Fsm,
    B: Fsm,
  {
    // Both machines have no further moves → this is a complete interleaving.
    if a.transitions(&sa).is_empty() && b.transitions(&sb).is_empty() {
      results.push(path.clone());
      return;
    }

    // Optional: skip product states we have already explored
    // (gives only simple paths through the product).
    if !visited.insert((sa.clone(), sb.clone())) {
      return;
    }

    // Advance machine A
    for next in a.transitions(&sa) {
      path.push(Step::A(next.clone()));
      dfs(a, b, next, sb.clone(), path, results, visited);
      path.pop();
    }

    // Advance machine B
    for next in b.transitions(&sb) {
      path.push(Step::B(next.clone()));
      dfs(a, b, sa.clone(), next, path, results, visited);
      path.pop();
    }

    visited.remove(&(sa, sb));
  }

  dfs(
    a,
    b,
    a.initial(),
    b.initial(),
    &mut path,
    &mut results,
    &mut visited,
  );

  results
}

// ---------------------------------------------------------------------------
// Example machines
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StateA {
  Start,
  Mid,
  End,
}

struct MachineA;

impl Fsm for MachineA {
  type State = StateA;

  fn initial(&self) -> Self::State {
    StateA::Start
  }

  fn transitions(&self, s: &Self::State) -> Vec<Self::State> {
    match s {
      StateA::Start => vec![StateA::Mid],
      StateA::Mid => vec![StateA::End],
      StateA::End => vec![],
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StateB {
  Ready,
  Done,
}

struct MachineB;

impl Fsm for MachineB {
  type State = StateB;

  fn initial(&self) -> Self::State {
    StateB::Ready
  }

  fn transitions(&self, s: &Self::State) -> Vec<Self::State> {
    match s {
      StateB::Ready => vec![StateB::Done],
      StateB::Done => vec![],
    }
  }
}

// ---------------------------------------------------------------------------
// Demo
// ---------------------------------------------------------------------------

fn main() {
  let a = MachineA;
  let b = MachineB;

  let sequences = interleave(&a, &b);

  println!("Found {} interleavings:\n", sequences.len());
  for (i, seq) in sequences.iter().enumerate() {
    print!("  {}: ", i + 1);
    for step in seq {
      match step {
        Step::A(s) => print!("A({:?}) ", s),
        Step::B(s) => print!("B({:?}) ", s),
      }
    }
    println!();
  }
}
