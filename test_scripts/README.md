# Cocktail API Test Scripts

This folder contains some short shell scripts which use curl to test the various endpoints of the Cocktail API,
either the service written in Python or the one in Rust.

The scripts are designed to be run in sequence:

new_cocktail.sh

| Step | Script             | Description |
|------|--------------------|-------------|
| 1    | new_cocktail.sh    | Create a new cocktail with the slug `whiskey-old-fashioned`  |
| 2    | get_cocktail.sh    | Retrieve the `whiskey-old-fashioned` from the service.        |
| 3    | update_cocktail.sh | Modify the slug and name of the `whiskey-old-fashioned`, making it a `whiskey-new-fashioned`  |
| 4    | delete_cocktail.sh | Delete the `whiskey-new-fashioned`  |
| 5    | list_cocktail.sh   | List page 4 of the cocktail recipes.  |

Be aware that if you attempt to run most of these scripts twice you'll either get a duplicate key error, because you're attempting to re-use the slug, or a 404 because you're attempting to access a document that's already been deleted.