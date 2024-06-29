Project Trailmap
================

(Because "roadmap" would be overstating it.)

Fundamentals
------------

Like, really basic, because there's not even an app here yet.

- [ ] basic "now" view
- [ ] basic card component
- [ ] ui for adding tasks
- [ ] server-side storage (turbosql?)
- [ ] archive
- [ ] trash
- [ ] backlog ("primordial soup")
- [ ] up next / this week
- [ ] manually populate this week from up next
- [ ] manually populate today from this week
- [ ] "defer" button (initially, kicks card back)

Future
------

### Soon

- [ ] convert tasks to notes (and back again)
- [ ] urgency: on fire, today, next few days, soon-ish, whenever
- [ ] importance: strategic focus, responsibility, nice-to-have, idea
- [ ] energy level
- [ ] UI for availble hours per week
- [ ] size (minutes, hour, few hours, a day, too big)
- [ ] pause and defer all "today" items at end of day
- [ ] clear this week (back to ready) on configured day of week

### Next

- [ ] auto-fill today and this week
- [ ] automatic reports (this week, arbitrary timeframe)
- [ ] clone / split cards
- [ ] review workflow
- [ ] authentication
- [ ] archive search

### Eventually

- [ ] get working time available each day from calendar
- [ ] "blocked on external" status

### Maybe

- [ ] dependencies?
- [ ] specific due dates?
- [ ] option for card urgancy to increase automatically?
- [ ] kanban view for big screens (columns: ready/up next/this week/today/done)
- [ ] repeating items (but don't allow them to pile up!)

Assorted Ideas and Notes
------------------------

### card data

Visible:

* title
* body
* tags (arbitrary? predefined?)
* context (work, home, etc)
* priority info:
  * urgency
  * importance
  * energy
* size (minutes,1 hour, 4 hours, too big)
  * "minutes" allocated 15 minutes, because context switch
* state: backlog / ready / this week / today / in progress / done / note


Hidden (but maybe a details view shows this stuff)

* created date
* last touched
* time spent in progress (used in report)
* reporting week? for notes entered _after_ the week is done?
* internal priority calculation

### auto-prioritization
* starts with urgent / important matrix
* weights "energy" by time since a fun task has been completed
* button to auto-fill week based on the above things fit into time available
  * initially have manual "hours available" for each day of the week and for each context
     * warn if there's not enough time left over!
  * eventually, get from "focus time" in google calendar
     * other calendars tbd
* same for today


### task review thoughts

* include periodically as a "minutes" task
* count number of times a task was deferred, and ask if those are
  realllly as urgent and important as given

### reports

* include both done and notes
* easy to hide items
* easy to add items (maybe each becomes a note?)
* checkbox to include time spent or not in exported view


### on GTD

1. "Collect" is great. No notes.
2. Process and review... ugghhhhh.
  * backlog grows to infinity
  * "2-minutes? just do it!" rule underestimates context switch cost