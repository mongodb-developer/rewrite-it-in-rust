# Flask Cocktail API

A simple API for getting cocktail recipes.

This code was written as part of a [blog post](https://developer.mongodb.com/),
it's not really designed to stand alone.

## Run the code

```bash

pip install -e .
FLASK_APP=cocktailapi flask run
```

## Developing

Run the following to install the project (and dev dependencies) into your active virtualenv:

```bash
pip install -e .[dev]
```

You can run the tests with:

```bash
pytest
```
