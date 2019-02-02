# maxbib

Simple terminal-based reference manager using YAML files to store library info.

# Blueprint
## Files
Libraries consist of a folder with a library.yaml file, and any associated files.
The library.yaml file contains all the information on the references.
All associated files have the sha1 hashes as file names.
## Searching
For searching, there is a 1024 character limit on the strings searched.
Split the searching up into multiple constraints, with a weighting on each of the constraints.

Eg. could have 3 constraints: 
- Title
- Authors
- Abstract

Then the weighting can be 1*Title + 0.5*Authors + 1*Abstract

These can be combined as well, eg. searching 
- Title + Year
- Journal + Authors
- Abstract
