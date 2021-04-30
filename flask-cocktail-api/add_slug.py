from enum import unique
import os
import re

from pymongo import MongoClient
from bson import ObjectId

client = MongoClient(os.environ["MONGO_URI"])

db = client.get_default_database()
recipes = db.get_collection("recipes")
recipes.create_index("slug", unique=True)


def slugify(name):
    return re.sub(r"[^a-z0-9]+", "-", name.lower())


for recipe in recipes.find():
    print(recipe["name"], slugify(recipe["name"]))
    recipes.update_one(
        {"_id": recipe["_id"]},
        {"$set": {"slug": slugify(recipe["name"])}},
    )
