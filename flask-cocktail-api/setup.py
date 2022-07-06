from setuptools import setup, find_packages

EXTRAS_REQUIRE = {
    "tests": [
        "pytest",
        "coverage[toml]>=5.0.2",
    ],
}
EXTRAS_REQUIRE["dev"] = EXTRAS_REQUIRE["tests"] + [
    "black",
    "twine",
    "wheel",
    "prospector[with_everything]",
]

setup(
    name="flask-cocktail-api",
    description="A small API for getting cocktail recipes.",
    version="0.0.0",
    author="Mark Smith",
    author_email="mark.smith@mongodb.com",
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    install_requires=[
        "fastapi == 0.63.0",
        "Flask == 1.1.4",
        "Flask-PyMongo==2.3.0",
        "pymongo[srv] == 3.11.3",
        "pydantic == 1.8.1",
        "MarkupSafe == 2.0.1"
    ],
    extras_require=EXTRAS_REQUIRE,
)