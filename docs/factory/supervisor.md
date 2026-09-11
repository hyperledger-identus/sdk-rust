# Supervisor-to-Pi runs

The supervisor owns admission, liveness, process cleanup, effect validation,
metrics, CI observation and every remote action. A Pi worker owns one declared
task and cannot grant itself merge, release, publication, cleanup or downstream
authority.

The closed records are:

- [`supervisor-invocation-v1.schema.json`](supervisor-invocation-v1.schema.json):
  exact repository, issue, profile, `origin/develop` base, branch/head, active
  preflight receipt digest, role, task, path/tool allowlists and deadlines;
- [`supervisor-heartbeat-v1.schema.json`](supervisor-heartbeat-v1.schema.json):
  supervisor time and process state only;
- [`worker-handoff-v1.schema.json`](worker-handoff-v1.schema.json): exact
  identity, changed paths, closed acceptance/check/finding outcomes, remaining
  worker-owned process count and a safe next action; and
- [`pi-usage-v1.schema.json`](pi-usage-v1.schema.json): aggregate Pi v3
  sessions, turns, tools and non-overlapping token buckets only.

## Run lifecycle

Prepare only after the planning receipt is valid. Repeat `--allow-path` and
`--tool` for the one worker role:

```bash
scripts/factory supervisor prepare \
  --change <change> --issue <number> --profile production-ready \
  --role developer --task <task-number> \
  --allow-path <repository-relative-prefix> \
  --tool read \
  --deadline-seconds 1200 --post-artifact-grace-seconds 30
```

Preparation creates an owner-only run directory below the Git common
directory and prints only the private invocation path. The supervisor pipes its
bounded worker input to standard input; it does not put that content in the
command line:

```bash
scripts/factory supervisor run --envelope <absolute-private-invocation-path>
```

The runner invokes Pi only as `./bootstrap.sh --pi`, persists the Pi session and
JSON event stream inside the private run, updates the heartbeat independently,
hardens Pi-created session files to owner-only permissions at exit, and
terminates the process group at the hard deadline. If a handoff appears
while the worker remains active, the post-artifact grace bounds process exit.
The command reports worker exit and handoff acceptance as separate closed
states without replaying worker output.

The wrapper prepends a bounded supervisor contract to the standard-input task.
The worker receives `SDK_FACTORY_INVOCATION` and `SDK_FACTORY_HANDOFF`; it must
write the handoff atomically with owner-only permissions. The supervisor then
compares the handoff identity to the invocation and current receipt, requires
an unchanged Git head, compares declared changed paths to current Git effects,
checks every changed path against the invocation prefixes, and verifies that no
worker-owned process remains.

The child environment removes GitHub token variables, points `gh` at an empty
owner-only configuration directory and gives `origin` a loopback-only push URL.
This blocks normal accidental GitHub and Git push mutation while leaving remote
coordination, publication and merge with the supervisor. It is defense in
depth, not hostile-process isolation: an unrestricted shell can deliberately
replace its environment.

```bash
scripts/factory supervisor handoff-validate \
  --envelope <absolute-private-invocation-path>
scripts/factory supervisor harvest \
  --envelope <absolute-private-invocation-path>
```

Harvesting accepts exactly one bounded regular Pi version 3 session plus its
bounded regular event stream. Terminal persisted assistant records supply the
non-overlapping input, output, cache-read and cache-write counters. Matched
turn/tool lifecycle events are counted once. Unsupported, incomplete or absent
sources become closed unavailable reasons; unsafe files fail.

Harvest is idempotent for the same exact run. A repeated harvest returns the
existing equal aggregate and rejects any conflicting retained record.

## Privacy and limitations

Session and event parsing is content-aware only long enough to extract
counters. The aggregate does not retain or emit prompts, messages, transcript
content, session/response/tool identifiers, commands, raw output,
provider/model fields, cost, credentials or billing data. Do not display or
copy the private session, event or stderr files into issues, logs, handoffs or
metrics.

Tool allowlists and post-run Git validation are not an operating-system
sandbox. A role allowed to use `bash` has the local operator's process and
filesystem permissions. Use a separate OS isolation boundary when hostile code
requires one.
