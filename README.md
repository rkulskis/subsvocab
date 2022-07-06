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
        - API_2 that accepts .net file containing all subtitles
            -> converts to .txt
            -> passes to API_3
        - API_3 parses .txt from API_2 and removes all non-language
          elements such as html body, leaving only subtitle vocabulary
            -> passes to API_4
            awk  '{gsub(/</,"\n\<"); gsub(/>/, ">\n"); print}' ipv4-c093-bos001-ix.1.oca.nflxvideo.net | grep -ov "<.*>" | tr -d '[:punct:]\|[:digit:]' | tr -s '\n' | tr '\n' ' '
        - API_4 accepts .txt from API_3 and formats as JSON 
        - API_5 accepts JSON from API_4 and reduces list by removing
          repeats and adding on traits to root words:
            -> num: number of times it shows up in script
            -> translation: translation to english
                **by default english, potentially add support for other
                  languages
          returns this JSON response to web app which displays it

APPROACH:
    (1) design API_2-API5 first, then worry about API_1
    (2) then make front end, but first just barebones API creation
