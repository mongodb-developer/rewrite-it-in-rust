# Sample Data

This folder contains sample data for the Cocktails API apps.
This data is designed to be used to complement an accompanying blog post.
The data was exported using [mongoexport](https://docs.mongodb.com/v4.0/reference/program/mongoexport/)
and can be imported into your own database using [mongoimport](https://docs.mongodb.com/v4.0/reference/program/mongoimport/).

Use the following command in this directory to import the collection into your database
(be aware that currently the sample code depends on the database being called "cocktails" and the collection "recipes").

```bash

mongoimport --uri YOUR-ATLAS-DB-URI --file ./recipes.json
```
