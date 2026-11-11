#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MyErr {
  JobError
}

struct TaskTransition {
  d_currStatus        : Status,
  d_nextStatus        : Status,
  d_parentStatus      : Status,
  d_nextParentStatus  : Option<Status>,
}

#[repr(u32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
  Queued,
  Reserved,
  Invoked,
  Running,
  Complete,
  Failed,
  Invalid,
}

impl Status {
  const taskTransition: [TaskTransition; 8] = [
    TaskTransition {
      d_currStatus        : Status::Reserved,
      d_nextStatus        : Status::Invoked,
      d_parentStatus      : Status::Reserved,
      d_nextParentStatus  : Some(Status::Running),
    },
    TaskTransition {
      d_currStatus        : Status::Reserved,
      d_nextStatus        : Status::Invoked,
      d_parentStatus      : Status::Running,
      d_nextParentStatus  : None,
    },
    TaskTransition {
      d_currStatus        : Status::Reserved,
      d_nextStatus        : Status::Invalid,
      d_parentStatus      : Status::Reserved,
      d_nextParentStatus  : Some(Status::Invalid),
    },
    TaskTransition {
      d_currStatus        : Status::Reserved,
      d_nextStatus        : Status::Invalid,
      d_parentStatus      : Status::Running,
      d_nextParentStatus  : Some(Status::Invalid),
    },
    TaskTransition {
      d_currStatus        : Status::Invoked,
      d_nextStatus        : Status::Running,
      d_parentStatus      : Status::Running,
      d_nextParentStatus  : None,
    },
    TaskTransition {
      d_currStatus        : Status::Invoked,
      d_nextStatus        : Status::Invalid,
      d_parentStatus      : Status::Running,
      d_nextParentStatus  : Some(Status::Invalid),
    },
    TaskTransition {
      d_currStatus        : Status::Running,
      d_nextStatus        : Status::Complete,
      d_parentStatus      : Status::Running,
      d_nextParentStatus  : None,
    },
    TaskTransition {
      d_currStatus        : Status::Running,
      d_nextStatus        : Status::Failed,
      d_parentStatus      : Status::Running,
      d_nextParentStatus  : Some(Status::Failed),
    },
  ];

  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Queued =>   "QUEUED",
      Self::Reserved => "RESERVED",
      Self::Invoked =>  "INVOKED",
      Self::Running =>  "RUNNING",
      Self::Complete => "COMPLETE",
      Self::Failed =>   "FAILED",
      Self::Invalid =>  "INVALID",
    }
  }

  pub fn from_str(value: &str) -> Option<Status> {
    match value {
      "QUEUED" =>   Some(Self::Queued),
      "RESERVED" => Some(Self::Reserved),
      "INVOKED" =>  Some(Self::Invoked),
      "RUNNING" =>  Some(Self::Running),
      "COMPLETE" => Some(Self::Complete),
      "FAILED" =>   Some(Self::Failed),
      "INVALID" =>  Some(Self::Invalid),
      _ => None,
    }
  }

  pub fn from_u32(value: u32) -> Option<Status> {
    match value {
      0 => Some(Self::Queued),
      1 => Some(Self::Reserved),
      2 => Some(Self::Invoked),
      3 => Some(Self::Running),
      4 => Some(Self::Complete),
      5 => Some(Self::Failed),
      6 => Some(Self::Invalid),
      _ => None,
    }
  }

  fn updateAsJob(&mut self, newStatus: Status) -> Result<(), MyErr> {
    match *self {
      Status::Queued    if newStatus==Status::Reserved  => *self = newStatus,
      Status::Reserved  if newStatus==Status::Invoked   => *self = newStatus,
      Status::Reserved  if newStatus==Status::Invalid   => *self = newStatus,
      Status::Invoked   if newStatus==Status::Running   => *self = newStatus,
      Status::Invoked   if newStatus==Status::Invalid   => *self = newStatus,
      Status::Running   if newStatus==Status::Complete  => *self = newStatus,
      Status::Running   if newStatus==Status::Failed    => *self = newStatus,
      _ => {
        *self = Status::Invalid;
        return Err(MyErr::JobError);
      },
    }
    return Ok(());
  }

  fn updateAsParent(&mut self, newStatus: Status) -> Result<(), MyErr> {
    match *self {
      Status::Queued    if newStatus==Status::Reserved  => *self = newStatus,
      Status::Reserved  if newStatus==Status::Running   => *self = newStatus,
      Status::Reserved  if newStatus==Status::Invalid   => *self = newStatus,
      Status::Running   if newStatus==Status::Complete  => *self = newStatus,
      Status::Running   if newStatus==Status::Failed    => *self = newStatus,
      Status::Running   if newStatus==Status::Invalid   => *self = newStatus,
      _ => {
        *self = Status::Invalid;
        return Err(MyErr::JobError);
      },
    }
    return Ok(());
  }

  fn updateAsTask(&mut self, newStatus: Status, parentStatus: &mut Status) -> Result<(), MyErr> {
    for i in 0..Status::taskTransition.len() {
      if *self == Status::taskTransition[i].d_currStatus && 
        newStatus == Status::taskTransition[i].d_nextStatus &&
        *parentStatus == Status::taskTransition[i].d_parentStatus {
        *self = newStatus;
        if let Some(status) = Status::taskTransition[i].d_nextParentStatus {
          return parentStatus.updateAsParent(status);
        } else {
          return Ok(());
        }
      }
    }
    // Bad transition: everything now invalid
    *self = Status::Invalid;
    *parentStatus = Status::Invalid;
    return Err(MyErr::JobError);
  }
}

struct SimpleTest {
  curr  : Status,
  next  : Status,
}

struct ComplexTest {
  curr      : Status,
  next      : Status,
  prnt      : Status,
  nextPrnt  : Option<Status>,
}

const jobTest: [SimpleTest; 7] = [
  SimpleTest{curr: Status::Queued, next: Status::Reserved,},
  SimpleTest{curr: Status::Reserved, next: Status::Invoked,},
  SimpleTest{curr: Status::Reserved, next: Status::Invalid,},
  SimpleTest{curr: Status::Invoked, next: Status::Running,},
  SimpleTest{curr: Status::Invoked, next: Status::Invalid,},
  SimpleTest{curr: Status::Running, next: Status::Complete,},
  SimpleTest{curr: Status::Running, next: Status::Failed,},
];

const parentTest: [ComplexTest; 8] = [
  ComplexTest{curr: Status::Reserved, next: Status::Invoked, prnt: Status::Reserved, nextPrnt: Some(Status::Running),},
  ComplexTest{curr: Status::Reserved, next: Status::Invoked, prnt: Status::Running, nextPrnt: None,},
  ComplexTest{curr: Status::Reserved, next: Status::Invalid, prnt: Status::Reserved, nextPrnt: Some(Status::Invalid),},
  ComplexTest{curr: Status::Reserved, next: Status::Invalid, prnt: Status::Running, nextPrnt: Some(Status::Invalid),},
  ComplexTest{curr: Status::Invoked, next: Status::Running, prnt: Status::Running, nextPrnt: None,},
  ComplexTest{curr: Status::Invoked, next: Status::Invalid, prnt: Status::Running, nextPrnt: Some(Status::Invalid),},
  ComplexTest{curr: Status::Running, next: Status::Complete, prnt: Status::Running, nextPrnt: None,},
  ComplexTest{curr: Status::Running, next: Status::Failed, prnt: Status::Running, nextPrnt: Some(Status::Failed),},
];

fn testJobStatus() {
  let max = (Status::Invalid as usize) + 1;
  for i in 0..max {
    for j in 0..max {
      let mut curr = Status::from_u32(i as u32).unwrap();
      let next = Status::from_u32(j as u32).unwrap();
      let mut found = false;
      println!("testJobStatus ({},{}) = ({:?},{:?})", i, j, curr, next); 
      for k in 0..jobTest.len() {
        if curr==jobTest[k].curr && next==jobTest[k].next {
          found = true;
          break;
        }
      }
      if found {
        assert_eq!(Ok(()), curr.updateAsJob(next));
        assert_eq!(next, curr);
      } else {
        assert_eq!(Err(MyErr::JobError), curr.updateAsJob(next));
        assert_eq!(Status::Invalid, curr);
      }
    }
  }
}

fn testParentStatus() {
  let max = (Status::Invalid as usize) + 1;
  for i in 0..max {
    for j in 0..max {
      for k in 0..max {
        let mut curr = Status::from_u32(i as u32).unwrap();
        let next = Status::from_u32(j as u32).unwrap();
        let mut prnt = Status::from_u32(k as u32).unwrap();
        let mut sidx: usize = 0;
        let mut found = false;
        println!("testParentStatus ({},{},{}) = ({:?},{:?},{:?})", i, j, k, curr, next, prnt); 
        for s in 0..parentTest.len() {
          if curr==parentTest[s].curr && next==parentTest[s].next && prnt==parentTest[s].prnt {
            sidx = s;
            found = true;
            break;
          }
        }
        if found {
          let prntCopy = prnt;
          assert_eq!(Ok(()), curr.updateAsTask(next, &mut prnt));
          assert_eq!(next, curr);
          if let Some(expectedNextPrnt) = parentTest[sidx].nextPrnt {
            assert_eq!(expectedNextPrnt, prnt);
          } else {
            assert_eq!(prntCopy, prnt);
          }
        } else {
          assert_eq!(Err(MyErr::JobError), curr.updateAsTask(next, &mut prnt));
          assert_eq!(Status::Invalid, curr);
          assert_eq!(Status::Invalid, prnt);
        }
      }
    }
  }
}

fn testSharedParentStatus() {
}
          
fn main() {
  testJobStatus();
  testParentStatus();
  testSharedParentStatus();
}
