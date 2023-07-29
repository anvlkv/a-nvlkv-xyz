# Purpose: Create a dataset from the portfolio folder

from datasets import Dataset, Image
from tomllib import loads
import os

def load_portfolio(dir_name):
    portfolio = []
    for entry in os.listdir(dir_name):
        if os.path.isdir(os.path.join(dir_name, entry)):
            entry_dict = {
                "name": entry,
                "images": [],
            }
            for file in os.listdir(os.path.join(dir_name, entry)):
                if file.endswith(".md"):
                    with open(os.path.join(dir_name, entry, file), "r") as f:
                        text = f.read()
                    [__, config, text] = text.strip().split("+++")
                    config = loads(config)
                    entry_dict["text"] = text
                    if config.__contains__("title"):
                      entry_dict["title"] = config["title"]
                    else:
                      entry_dict["title"] = "untitled"
                    entry_dict["category"] = config["taxonomies"]["category"]
                elif file.endswith(".png") | file.endswith(".jpg") | file.endswith(".jpeg"):
                    entry_dict["images"].append(os.path.join(dir_name, entry, file))

            portfolio.append(entry_dict)
    
    ds_dict = {
        "text": [],
        "images": [],
    }

    for entry in portfolio:
        for img in entry["images"]:
            ds_dict["text"].append(f"{','.join(entry['category'])}: {entry['title']}")
            ds_dict["images"].append(img)
        
    dataset = Dataset.from_dict(ds_dict).cast_column("images", Image())
    return dataset

portfolio = load_portfolio("./content/portfolio")

portfolio.save_to_disk("./datasets/portfolio")


print(portfolio.num_rows)