use DecayPtr;
use EndScheduleTrait;
use Scheduler;
use SchedulerTrait;
use TaskBox;

pub struct EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                          TScheduler>
    where TScheduler: 'a +
                      SchedulerTrait {
    scheduler: &'a TScheduler,
    task_boxes: Vec<TaskBox>,
    continuation_box: TaskBox,
}

impl <'a,
      TScheduler> EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                                 TScheduler>
    where TScheduler: SchedulerTrait {
    pub fn new(scheduler: &'a TScheduler,
               task_boxes: Vec<TaskBox>,
               continuation_box: TaskBox) -> EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                                                            TScheduler> {
        EndScheduleMultipleTaskBoxesOneContinuationBox { scheduler: scheduler,
                                                         task_boxes: task_boxes,
                                                         continuation_box: continuation_box }
    }
}

impl <'a,
      TScheduler> EndScheduleTrait for EndScheduleMultipleTaskBoxesOneContinuationBox<'a,
                                                                                      TScheduler>
    where TScheduler: SchedulerTrait {
    type TEndScheduleReturn = TScheduler::TScheduleMultipleReturn;

    fn end_schedule(self) -> Self::TEndScheduleReturn {
        let task_boxes = self.task_boxes;
        let continuation_box = self.continuation_box;

        let decaying_continuation_box = unsafe { DecayPtr::new(continuation_box) };

        let mut result_tasks = Vec::<TaskBox>::new();

        for task_box in task_boxes {
            let current_decaying_continuation_box = unsafe { decaying_continuation_box.clone() };

            let current_task = move |scheduler: &Scheduler| {
                task_box.call_box((&scheduler,));
                match unsafe { current_decaying_continuation_box.decay() } {
                    Some(continuation_box) => {
                        scheduler.schedule(continuation_box);
                    },
                    None => {
                        // Do nothing
                    },
                }
            };
            let current_task_box = Box::new(current_task);

            result_tasks.push(current_task_box);
        }

        self.scheduler.schedule_multiple(result_tasks)
    }
}
