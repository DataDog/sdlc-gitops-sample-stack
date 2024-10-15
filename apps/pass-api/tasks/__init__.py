# https://github.com/pyinvoke/invoke/issues/946
# mypy: disable-error-code="arg-type"

"""
Invoke entrypoint, import here all the tasks we want to make available
"""

from invoke import Collection, Task

from tasks.go import (generate_licenses)
from tasks.custom_task.custom_task import custom__call__
from tasks.libs.common.go_workspaces import handle_go_work


Task.__call__ = custom__call__

# the root namespace
ns = Collection()

# add single tasks to the root
ns.add_task(generate_licenses)

ns.configure(
    {
        "run": {
            # this should stay, set the encoding explicitly so invoke doesn't
            # freak out if a command outputs unicode chars.
            "encoding": "utf-8",
        }
    }
)

# disable go workspaces by default
handle_go_work()
