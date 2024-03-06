from datetime import timedelta

from oxypy import Option, Result

PROFILES = ["dev", "release"]

def create_bench_command(profile: str) -> Result[timedelta, str]:
    if profile not in PROFILES:
        return Result.err("invalid profile name")
    
    cmd = f"cargo bench --profile {profile}"

    return Result.ok(cmd)


def main():
    cmd = create_bench_command("release")

    cmd.unwrap_or(timedelta(0))