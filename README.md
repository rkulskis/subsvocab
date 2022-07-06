# subsvocab
Web app for generating vocabulary lists from Netflix subtitles. The list will be sorted by how many times each word shows up.

REQUIREMENTS ANALYSIS:
    USER should be able to:
        - select netflix episode with subtitles or CC to pass into web app
          and generate vocab list
        - experience should be fast, reliable, secure
            -> Rust + webassembly
        - if time permits (I need to show this by 8/13 and today is 8/1), then
          also generate a backend database for the user to save vocab lists
            -> mySQL?
        - first should be able to handle just Spanish
            -> after this add on support for other languages using lingua 
               rust crate

DESIGN + ARCHITECTURE:
    - microservices architecture
    COMPONENTS:
    FRONTEND:
        INPUT FIELDS:
            - input_language
            - output_language   ** these may not be necessary if implement
                                   language detection crate
        maybe it can be a chrome extension and take the user to a new
        page displaying the JSON containing vocab

    BACKEND:
        - API_1 that fetches .net file using parser4curls rust crate
            -> accepts user credentials?
            -> alternatively user could download file OR load page and
               this service could perform the cURL operations to get .net
               file from loaded page
            -> passes .net to API_2
        - API_2 parses + puts words into list/datastructure
        - API_3 converts into JSON
        - API_4 accepts JSON from API_3 and reduces list by removing
          repeats and adding on traits to root words:
            -> num: number of times it shows up in script
            -> translation: translation to english (https://crates.io/crates/libretranslate)
                **by default english, potentially add support for other
                  languages
            -> get rid of things that are the same in both languages
          returns this JSON response to web app which displays it

APPROACH:
    (1) design API_2-API5 first, then worry about API_1
    (2) then make front end, but first just barebones API creation
