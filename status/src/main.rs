#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MyErr {
  JobError
}

struct TaskTransition<'a> {
  d_currStatus        : Status,
  d_nextStatus        : Status,
  d_parentStatus      : &'a [Status],
  d_nextParentStatus  : Option<Status>,
}

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
  const taskTransition: [TaskTransition<'_>; 6] = [
    TaskTransition {
      d_currStatus        : Status::Reserved,
      d_nextStatus        : Status::Invoked,
      d_parentStatus      : &[Status::Reserved, Status::Running],
      d_nextParentStatus  : Some(Status::Running),
    },
    TaskTransition {
      d_currStatus        : Status::Reserved,
      d_nextStatus        : Status::Invalid,
      d_parentStatus      : &[Status::Reserved, Status::Running],
      d_nextParentStatus  : Some(Status::Invalid),
    },
    TaskTransition {
      d_currStatus        : Status::Invoked,
      d_nextStatus        : Status::Running,
      d_parentStatus      : &[Status::Running],
      d_nextParentStatus  : None,
    },
    TaskTransition {
      d_currStatus        : Status::Invoked,
      d_nextStatus        : Status::Invalid,
      d_parentStatus      : &[Status::Running],
      d_nextParentStatus  : Some(Status::Invalid),
    },
    TaskTransition {
      d_currStatus        : Status::Running,
      d_nextStatus        : Status::Complete,
      d_parentStatus      : &[Status::Running],
      d_nextParentStatus  : None,
    },
    TaskTransition {
      d_currStatus        : Status::Running,
      d_nextStatus        : Status::Failed,
      d_parentStatus      : &[Status::Running],
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
    for srcIdx in 0..Status::taskTransition.len() {
      if *self == Status::taskTransition[srcIdx].d_currStatus && 
        newStatus == Status::taskTransition[srcIdx].d_nextStatus {
        for prtIdx in 0..Status::taskTransition[srcIdx].d_parentStatus.len() {
          if *parentStatus == Status::taskTransition[srcIdx].d_parentStatus[prtIdx] {
            *self = newStatus;
            if let Some(status) = Status::taskTransition[srcIdx].d_nextParentStatus {
              return parentStatus.updateAsParent(status);
            }
            return Ok(());
          }
        }
      }
    }
    // Bad transition: everything now invalid
    *self = Status::Invalid;
    *parentStatus = Status::Invalid;
    return Err(MyErr::JobError);
  }
}

fn main() {
  let mut t1: Status;
  let mut parentStatus: Status;

  // Case 1: task happy path Reserved -> Invoked -> Running -> Complete
  // Task Reserved -> Invoked
  t1 = Status::Reserved;
  parentStatus = Status::Queued;
  assert_eq!(Ok(()), parentStatus.updateAsParent(Status::Reserved));
  assert_eq!(Status::Reserved, parentStatus);
  assert_eq!(Ok(()), t1.updateAsTask(Status::Invoked, &mut parentStatus));
  assert_eq!(Status::Invoked, t1);
  assert_eq!(Status::Running, parentStatus);
  // Task Invoked -> Running
  assert_eq!(Ok(()), t1.updateAsTask(Status::Running, &mut parentStatus));
  assert_eq!(Status::Running, t1);
  assert_eq!(Status::Running, parentStatus);
  // Task Running -> Complete
  assert_eq!(Ok(()), t1.updateAsTask(Status::Complete, &mut parentStatus));
  assert_eq!(Status::Complete, t1);
  assert_eq!(Status::Running, parentStatus);

  // Case 2: task error case Reserved -> Invalid
  t1 = Status::Reserved;
  parentStatus = Status::Queued;
  assert_eq!(Ok(()), parentStatus.updateAsParent(Status::Reserved));
  assert_eq!(Ok(()), t1.updateAsTask(Status::Invalid, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  // Once parentStatus invalid all transitions must fail
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Queued));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Reserved));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Invoked));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Running));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Complete));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Failed));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Invalid));
  assert_eq!(Status::Invalid, parentStatus);
  // Once t1 invalid all transitions must fail
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Queued, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Reserved, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Invoked, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Running, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Complete, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Failed, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Invalid, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);

  // Case 3: task error case Reserved -> Invoked -> Invalid
  t1 = Status::Reserved;
  parentStatus = Status::Queued;
  assert_eq!(Ok(()), parentStatus.updateAsParent(Status::Reserved));
  assert_eq!(Status::Reserved, parentStatus);
  // task Reserved -> Invoked
  assert_eq!(Ok(()), t1.updateAsTask(Status::Invoked, &mut parentStatus));
  assert_eq!(Status::Invoked, t1);
  assert_eq!(Status::Running, parentStatus);
  // task Invoked -> Invalid
  assert_eq!(Ok(()), t1.updateAsTask(Status::Invalid, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  // Once parentStatus invalid all transitions must fail
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Queued));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Reserved));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Invoked));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Running));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Complete));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Failed));
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Invalid));
  assert_eq!(Status::Invalid, parentStatus);
  // Once t1 invalid all transitions must fail
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Queued, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Reserved, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Invoked, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Running, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Complete, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Failed, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Invalid, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);

  // Case 4: task error case Reserved -> Invoked -> Running -> Failed
  t1 = Status::Reserved;
  parentStatus = Status::Queued;
  assert_eq!(Ok(()), parentStatus.updateAsParent(Status::Reserved));
  assert_eq!(Status::Reserved, parentStatus);
  // task Reserved -> Invoked
  assert_eq!(Ok(()), t1.updateAsTask(Status::Invoked, &mut parentStatus));
  assert_eq!(Status::Invoked, t1);
  assert_eq!(Status::Running, parentStatus);
  // task Invoked -> Running
  assert_eq!(Ok(()), t1.updateAsTask(Status::Running, &mut parentStatus));
  assert_eq!(Status::Running, t1);
  assert_eq!(Status::Running, parentStatus);
  // task Running -> Failed
  assert_eq!(Ok(()), t1.updateAsTask(Status::Failed, &mut parentStatus));
  assert_eq!(Status::Failed, t1);
  assert_eq!(Status::Failed, parentStatus);

  // Once parentStatus Failed all transitions must fail. There are two things
  // to check. First, updateAsParent must return an error. Second, because the
  // transition from Failed -> x is invalid, parentStatus force set to Invalid
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Queued));
  assert_eq!(Status::Invalid, parentStatus);
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Reserved));
  assert_eq!(Status::Invalid, parentStatus);
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Invoked));
  assert_eq!(Status::Invalid, parentStatus);
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Running));
  assert_eq!(Status::Invalid, parentStatus);
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Complete));
  assert_eq!(Status::Invalid, parentStatus);
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Failed));
  assert_eq!(Status::Invalid, parentStatus);
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), parentStatus.updateAsParent(Status::Invalid));
  assert_eq!(Status::Invalid, parentStatus);
  parentStatus = Status::Failed;
  // Once t1 Failed all transitions must fail. There are three things to check.
  // First, t1.updateAsTask must return an error. Second, because the transition
  // from Failed -> x is invalid, t1 force set to Invalid. When t1 force set to
  // Invalid, so is its parent force set Invalid
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Queued, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  t1 = Status::Failed;
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Reserved, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  t1 = Status::Failed;
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Invoked, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  t1 = Status::Failed;
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Running, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  t1 = Status::Failed;
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Complete, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  t1 = Status::Failed;
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Failed, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  t1 = Status::Failed;
  parentStatus = Status::Failed;
  assert_eq!(Err(MyErr::JobError), t1.updateAsTask(Status::Invalid, &mut parentStatus));
  assert_eq!(Status::Invalid, t1);
  assert_eq!(Status::Invalid, parentStatus);
  t1 = Status::Failed;
  parentStatus = Status::Failed;
}
