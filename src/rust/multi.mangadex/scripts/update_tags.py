# curl https://api.mangadex.org/manga/tag | jq "[.data[] | { type: \"genre\", name: .attributes.name.en, id: .id, canExclude: true }]"
import subprocess
import json
import os
import shutil

if not shutil.which("curl"):
    raise Exception("curl is not installed")

tags = json.loads(subprocess.check_output(["curl", "-sL", "https://api.mangadex.org/manga/tag"]))
parsedTags = [
    {
        "type": "genre",
        "name": tag["attributes"]["name"]["en"],
        "id": tag["id"],
        "canExclude": True,
    }
    for tag in tags["data"]
]

filters_json = os.path.join(os.path.dirname(os.path.realpath(__file__)), "..", "res", "filters.json")
with open(filters_json, "r+") as f:
    filters = json.load(f)
    for filter in filters:
        if filter.get("name") == "Tags":
            filter["filters"] = parsedTags
    f.seek(0)
    json.dump(filters, f, indent=4)

