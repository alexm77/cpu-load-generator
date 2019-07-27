# cpu-load-generator
Calculates the value of PI on several threads at once

This mini-project was sparked by a forum discussion about the potential efficiency of hyper-threading (SMT).

The program spawns a number of threads, each computing the value of PI over a fixed number of iteration. It uses the Monte Carlo method which is probably the poorest choice, but we're not concerned about converging fast towards the correct value, but merely putting strain on thr CPU.

Since the unit of work per thread is fixed, the program can be run across all physical ot logical cores. If it takes longer to get the job done when using all cores, that extra time can be interpreted as HT/SMT
inefficiency (as opposed to using fully-fledged cores only); modern CPUs tend to boost a bit higher when not all cores are in use, so a slight overhead when using all cores is expected.

For the most accurate results, I would deactivate SMT when testing physical cores, that way you can ensure no two threads will run on the same physical core.
