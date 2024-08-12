#ifndef _SCHEDULE_H
#define _SCHEDULE_H

#include "logic.h"

typedef struct
{
  /* period in ticks */
  int period;
  /* ticks until next activation */
  int offset;
  /* function pointer */
  void (*func)(void);
  /* activation counter */
  int exec;
} Sched_Task_t;

/* defines max number of tasks supported */
#define MAXT 10

/* array of Task Control Blocks - TCBs */
Sched_Task_t Tasks[MAXT];

/* kernel initialization routine */
int Sched_Init(void)
{
  /* - Initialise data
    structures.
  */
  byte x;
  for (x = 0; x < MAXT; x++)
    Tasks[x].func = 0;
  /* note that "func" will be used to see if a TCB */
  /* is free (func=0) or used (func=pointer to task code) */
  /* - Configure interrupt
    that periodically
    calls
    Sched_Schedule().
  */

  return 0;
}

/* adding a task to the kernel */
int Sched_AddT(void (*f)(void), int d, int p)
{
  byte x;
  for (x = 0; x < MAXT; x++)
    if (!Tasks[x].func) {
      /* finds the first free TCB */
      Tasks[x].period = p;
      Tasks[x].offset = d; /* first activation is "d" after kernel start */
      Tasks[x].exec = 0;
      Tasks[x].func = f;
      return x;
    }
  return -1; /* if no free TCB --> return error */
}

/**
 * Kernel scheduler, just activates periodic tasks
 * "offset" is always counting down, activate task when 0
 * then reset to "period"
 * --> 1st activation at "offset" and then on every "period"
*/
void Sched_Schedule(void)
{
  byte x;
  for (x = 0; x < MAXT; x++)
  {
    if ((Tasks[x].func) && (Tasks[x].offset))
    {
      /* for all existing tasks (func!=0) and not at 0, yet */
      Tasks[x].offset--; /* decrement counter */
      if (!Tasks[x].offset) {
        /* offset = 0 --> Schedule Task --> set the "exec" flag/counter */
        /* Tasks[x].exec++; accummulates activations if overrun */
        Tasks[x].exec = 1; /* if overrun, following activation is lost */
        Tasks[x].offset = Tasks[x].period; /* reset counter */
      }
    }
  }
}

/**
 * Kernel dispatcher, taskes highest priority ready task and runs it
 * the distacher can only be called again upon task termination
 * (task is called within the dispatcher)
 * --> non-preemption
 */
void Sched_Dispatch(void) {
  /* index of currently running task (MAXT to force searching all TCBs initially) */
  byte cur_task = MAXT;
  byte x;

  for (x = 0; x < cur_task; x++) {
    /* x searches from 0 (highest priority) up to x (current task) */
    if ((Tasks[x].func) && (Tasks[x].exec)) {
      /* if a TCB has a task (func!=0) and there is a pending activation */
      Tasks[x].exec--; /* decrement (reset) "exec" flag/counter */

      Tasks[x].func(); /* Execute the task */

      /* Delete task if one-shot, i.e., only runs once (period=0 && offset!0) */
      if (!Tasks[x].period)
        Tasks[x].func = 0;
      return;
    }
  }
}

#endif
