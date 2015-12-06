use EndScheduleTrait;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleOneTaskBoxNoContinuations<'a,
                                                TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_box: TaskBox,
}

impl <'a,
      TScheduler> EndScheduleOneTaskBoxNoContinuations<'a,
                                                       TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_box: TaskBox) -> EndScheduleOneTaskBoxNoContinuations<'a,
                                                                          TScheduler> {
        EndScheduleOneTaskBoxNoContinuations { scheduler: scheduler,
                                               task_box: task_box }
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for EndScheduleOneTaskBoxNoContinuations<'a,
                                                                            TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        self.scheduler.schedule()
    }
}
