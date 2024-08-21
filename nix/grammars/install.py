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
    print("=> ERROR: missing tree-sitter in package.json")
    exit(1)

for config in package_json["tree-sitter"]:

    if "path" in config:
        grammar_subpath = Path(config["path"])
    else:
        grammar_subpath = Path(".")

    shutil.copytree(grammar_subpath / "src", out / grammar_subpath / "src", dirs_exist_ok = True)


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

