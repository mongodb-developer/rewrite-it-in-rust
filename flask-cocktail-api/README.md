# Flask Cocktail API

A simple API for getting cocktail recipes.

This code was written as part of a [blog post](https://developer.mongodb.com/),
it's not really designed to stand alone.

## Run the code

First create and activate your virtualenv - with the `venv` package on OSX or Linux, this will be:

```bash
python3 -m venv venv
source venv/bin/activate
```

With your virtualenv active, install the project locally:

```bash
pip install -e .
```

You'll need to set the environment variable `MONGO_URI` to the URL of your MongoDB replicaset.
I use `direnv` to configure this, and put the following line in my `.envrc` file in my project's directory:

```bash
export MONGO_URI="mongodb+srv://USERNAME:PASSW0RD@cluster0-abcde.azure.mongodb.net/cocktails?retryWrites=true&w=majority"
```

You can then import the sample data with mongoimport:

```bash
mongoimport --uri "$MONGO_URI" --file ../data/recipes.json
```

And now you should be able to run the service like this:

```bash
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
