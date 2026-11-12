#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MyErr {
  JobError
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

  const fn jobInitialState() -> Status {
    return Status::Queued;
  }

  const fn parentInitialState() -> Status {
    return Status::Reserved;
  }

  const fn taskInitialState() -> Status {
    return Status::Reserved;
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

  fn updateAsTask(&mut self, newStatus: Status) -> Result<(), MyErr> {
    match *self {
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

  fn updateAsParent(&mut self, taskStatus: &Vec<Status>) -> Result<(), MyErr> {
    let mut isInvalid = false;
    let mut isFailed  = false;
    let mut isComplete = true;
    let mut invokedCount: usize = 0;
    let mut reservedCount: usize = 0;
    let mut needTransition = false;
    let mut nextStatus = Status::Invalid;

    // Summarize task statuses
    for i in 0..taskStatus.len() {
      // Count tasks in Invoked state
      invokedCount += (taskStatus[i]==Status::Invoked) as usize;
      // Count tasks in Reserve state
      reservedCount += (taskStatus[i]==Status::Reserved) as usize;
      // Any invalid task makes parent Invalid
      isInvalid = isInvalid || taskStatus[i]==Status::Invalid;
      // Any failed task makes parent Failed provided no task Invalid
      isFailed = isFailed || taskStatus[i]==Status::Failed;
      // Parent is complete if all tasks Complete
      isComplete = isComplete && taskStatus[i]==Status::Complete;
    }

    // println!("Computed final: invokedCount {} reservedCount {} isInvalid {} isFailed {} isComplete {}",
    //  invokedCount, reservedCount, isInvalid, isFailed, isComplete);

    if *self==Status::Reserved && invokedCount==1 && reservedCount==taskStatus.len()-1 {
      // First task invocation transitions parent Reserved -> Running
      needTransition = true;
      nextStatus = Status::Running;
    } else if isInvalid  && *self != Status::Invalid {
      // Any Invalid task state makes parent x -> Invalid
      needTransition = true;
      nextStatus = Status::Invalid;
    } else if isFailed && !isComplete && *self != Status::Failed {
      // Any Failed task state makes parent x -> Failed provided not Invalid
      // That is, Invalid takes precedence over Failed
      needTransition = true;
      nextStatus = Status::Failed;
    } else if isComplete && !isFailed && *self != Status::Complete {
      // If all tasks Complete parent is Complete
      needTransition = true;
      nextStatus = Status::Complete;
    }

    // Validate and enforce 'nextStatus' as needed
    if needTransition {
      match *self {
        Status::Reserved if nextStatus==Status::Running  => *self = nextStatus,
        Status::Reserved if nextStatus==Status::Invalid  => *self = nextStatus,
        Status::Running  if nextStatus==Status::Complete => *self = nextStatus,
        Status::Running  if nextStatus==Status::Failed   => *self = nextStatus,
        Status::Running  if nextStatus==Status::Invalid  => *self = nextStatus,
        _ => {
          *self = Status::Invalid;
          return Err(MyErr::JobError);
        },
      }
    }
    return Ok(());
  }
}

struct JobTest {
  curr  : Status,
  next  : Status,
}

const jobTest: [JobTest; 7] = [
  JobTest{curr: Status::jobInitialState(), next: Status::Reserved,},
  JobTest{curr: Status::Reserved, next: Status::Invoked,},
  JobTest{curr: Status::Reserved, next: Status::Invalid,},
  JobTest{curr: Status::Invoked, next: Status::Running,},
  JobTest{curr: Status::Invoked, next: Status::Invalid,},
  JobTest{curr: Status::Running, next: Status::Complete,},
  JobTest{curr: Status::Running, next: Status::Failed,},
];

struct Transition {
  runLhs  : bool,
  next    : Status,
  prnt    : Status,
}

struct Trace<'a> {
  enabled     : bool,
  sequence    : &'a [Transition],
}

const traceTest: [Trace; 150] = [
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
  Trace{enabled: false, sequence: &[
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Running,},
  ],},
];

fn testJobStatus() {
  let max = (Status::Invalid as usize) + 1;
  for i in 0..max {
    for j in 0..max {
      let mut curr = Status::from_u32(i as u32).unwrap();
      let next = Status::from_u32(j as u32).unwrap();
      let mut found = false;
      println!("testJobStatus ({},{})", i, j); 
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

fn testSharedParentTaskStatus() {
  for i in 0..traceTest.len() {
    if !traceTest[i].enabled {
      continue;
    }
    println!("testSharedParentTaskStatus: run lhs case {}", i);
    let mut prnt = Status::parentInitialState();
    let mut taskStatus: Vec<Status> = vec![Status::taskInitialState(), Status::taskInitialState()];
    for j in 0..traceTest[i].sequence.len() {
      if traceTest[i].sequence[j].runLhs {
        assert_eq!(Ok(()), taskStatus[0].updateAsTask(traceTest[i].sequence[j].next));
        assert_eq!(Ok(()), prnt.updateAsParent(&taskStatus));
        assert_eq!(traceTest[i].sequence[j].next, taskStatus[0]);
        assert_eq!(traceTest[i].sequence[j].prnt, prnt);
      } else {
        assert_eq!(Ok(()), taskStatus[1].updateAsTask(traceTest[i].sequence[j].next));
        assert_eq!(Ok(()), prnt.updateAsParent(&taskStatus));
        assert_eq!(traceTest[i].sequence[j].next, taskStatus[1]);
        assert_eq!(traceTest[i].sequence[j].prnt, prnt);
      }
    }
  }
}

fn main() {
  testJobStatus();
  testSharedParentTaskStatus();
}
