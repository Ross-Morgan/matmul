from datetime import timedelta
from pprint import pprint

from oxypy import Option, Result

PROFILES = ["dev", "release"]

def create_bench_command(profile: str) -> Result[str, str]:
    if profile not in PROFILES:
        return Result.err("invalid profile name")
    
    cmd = f"cargo bench --profile {profile}"

    return Result.ok(cmd)


def main():
    cmds = list(map(lambda cmd: cmd.expect("failed to create command"), map(create_bench_command, PROFILES)))
    
    pprint(cmds)


if __name__ == "__main__":
    main()

