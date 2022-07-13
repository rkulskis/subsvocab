# subsvocab
Web app for generating JSON vocabulary lists from Netflix subtitles. The elements in the list are sorted based off of how many times they show up in the script.

# Support
Currently this project only supports generation of vocabulary lists that are spanish -> english (unidirectional)

# Method
The method used for this project is NOT natural language processing, but rather HashMap indexing. We have a hash map such that each key is a conjugated form of a word and they map to their root words. Those root words map to dictionary translation entities. So the program queries the HashMap each word of the netflix script and retrieves their root equivalents. These roots are then used to query the root translation of the original word in the netflix script.
