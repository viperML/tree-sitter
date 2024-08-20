import json
import os
from pathlib import Path
import shutil


try:
    out = Path(os.environ["out"])
except KeyError:
    print("Using /tmp/out for debugging purposes")
    out = Path("/tmp/out")

out.mkdir(parents=True, exist_ok=True)

with open("package.json", "r") as f:
    package_json = json.load(f)

if "tree-sitter" not in package_json:
    print("=> WARNING: missing tree-sitter in package.json")
    exit(0)

for config in package_json["tree-sitter"]:

    for thing in ["highlights", "injections", "locals"]:
        try:
            if isinstance(config[thing], list):
                for path in config[thing]:
                    path = Path(path)
                    print("Processing: ", path)
                    (out / path.parent).mkdir(parents=True, exist_ok=True)
                    shutil.copy(path, (out /  path))
            else:
                path = Path(config[thing])
                print("Processing: ", path)
                (out / path.parent).mkdir(parents=True, exist_ok=True)
                shutil.copy(path, (out / path))


        except KeyError:
            print(f"Couldn't find {thing}")

