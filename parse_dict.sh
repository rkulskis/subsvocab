#!/bin/sh

# this script accepts the .tei file in the form of an english-spanish dictionary,
# and creates a string such that each line contains (in the following order):
# (1) root enlgish translation (2) root version of word (3) every conjugated/declined version of word
# note that we IGNORE accentuation

# delete conjugated_dictionary if it exists
if [ -f "./conjugated_dict.txt" ]
then
	rm -rf ./conjugated_dict.txt
fi

# grep only the english and spanish entries in the dictionary, remove <quote> tags and remove all accentuation, and remove all multi-word expressions
# EN translations are in <orth> tags, ES root words are in <quote> tags
ENTRIES=$(grep -aE '<orth>|<quote>' $1 | grep -v "<quote>.* .*" |  sed 's/.*<quote>//g;s/<\/quote>//g;s/.*<orth>/<orth>/g' | iconv -f UTF-8 -t ASCII//TRANSLIT | tr -d "~'!?\"\.%")

indicative_present()
{
local word="$1"
local stem="$2"
local ending="$3" 

if [[ ${ending} == "a" ]] || [[ ${ending} == "e" ]]
then
	echo "${stem}o ${stem}${ending}s ${stem}${ending} ${stem}${ending}mos ${stem}${ending}is ${stem}${ending}n"
else 	# -ir verb
	echo "${stem}o ${stem}es ${stem}e ${stem}${ending}mos ${stem}is ${stem}en"
fi
}

indicative_imperfect()
{
local word="$1"
local stem="$2"
local ending="$3" 

if [[ ${ending} == "e" ]] || [[ ${ending} == "i" ]]
then
	echo "${stem}ia ${stem}ias ${stem}ia ${stem}iamos ${stem}iais ${stem}ian"
else
	echo "${stem}aba ${stem}abas ${stem}aba ${stem}abamos ${stem}abais ${stem}aban"
fi
}

preterite()
{
local word="$1"
local stem="$2"
local ending="$3" 

if [[ ${ending} == "e" ]] || [[ ${ending} == "i" ]]
then
	echo "${stem}i ${stem}iste ${stem}io ${stem}imos ${stem}isteis ${stem}ieron"
else
	echo "${stem}e ${stem}aste ${stem}o ${stem}amos ${stem}asteis ${stem}aron"
fi
}

future()
{
local word="$1"
local stem="$2"
local ending="$3" 

echo "${stem}e ${stem}as ${stem}a ${stem}emos ${stem}eis ${stem}an"
}

conditional()
{
local word="$1"
local stem="$2"
local ending="$3" 

echo "${stem}ia ${stem}ias ${stem}ia ${stem}iamos ${stem}iais ${stem}ian"
}

subjunctive_present()
{
local word="$1"
local stem="$2"
local ending="$3" 

if [[ ${ending} == "a" ]]
then
	echo "${stem}e ${stem}es ${stem}e ${stem}emos ${stem}eis ${stem}en"
else
	echo "${stem}a ${stem}as ${stem}a ${stem}amos ${stem}ais ${stem}an"
fi
}

subjunctive_imperfect()
{
local word="$1"
local stem="$2"
local ending="$3" 
[ "$ending" = "a" ] && local newending="a" || local newstem="ie"
# local newending=$(( "$ending" == "a" ? "${stem}a" : "${stem}ie"))

echo "${stem}${newending}ra ${stem}${newending}ras ${stem}${newending}ra ${stem}${newending}ramos ${stem}${newending}rais ${stem}${newending}ran"
}

subjunctive_future() {
local word="$1"
local stem="$2"
local ending="$3" 
[ "$ending" = "a" ] && local newending="a" || local newstem="ie"
# local newending=$(( "$ending" == "a" ? "${stem}a" : "${stem}ie"))

echo "${stem}${newending}re ${stem}${newending}res ${stem}${newending}re ${stem}${newending}remos ${stem}${newending}reis ${stem}${newending}ren"
}

# imperative at the moment not included

past_participle() {
local word="$1"
local stem="$2"
local ending="$3" 

if [[ ${ending} == "a" ]]
then
	echo "${stem}ado"
else
	echo "${stem}ido"
fi
}

gerund() {
local word="$1"
local stem="$2"
local ending="$3" 

if [[ ${ending} == "a" ]]
then
	echo "${stem}ando"
else
	echo "${stem}iendo"
fi
}

# rest of compound tenses formed using form of haber + participle, so we will
# omit them since we already covered the participle and this dictionary only
# contains singular words

# accepts a noun or adjective and returns entry with all declined forms 
noun_adj_handler ()
{
local word="$1"
if [[ ${word: -1} == "o" ]]	# -o suffix adj + nouns 
then
 	echo "${word}s ${word%?}a ${word%?}as"
elif [[ ${word: -1} == "a" ]] || [[ ${word: -1} == "e" ]] # -a and -e suffix adj + nouns
then
	echo "${word}s"
elif [[ ${word: -1} == z ]]	# -z suffix adj + nouns
then
	echo "${word%?}ces"
else
	echo "${word}s ${word}es" # some incorrect words, but doesn't matter since these map to one definition
fi
}

# accepts verb and returns entry with all conjugated forms
verb_handler () 
{
local word="$1"
local stem="${word%??}"
local ending="${word: -2:1}"	# determines whether -ar, -er, or -ir verb

INDICATIVE_PRESENT=$(indicative_present "$word" "$stem" "$ending")
INDICATIVE_IMPERFECT=$(indicative_imperfect "$word" "$stem" "$ending")
PRETERITE=$(preterite "$word" "$stem" "$ending")
FUTURE=$(future "$word" "$stem" "$ending")
CONDITIONAL=$(conditional "$word" "$stem" "$ending")
SUBJUNCTIVE_PRESENT=$(subjunctive_present "$word" "$stem" "$ending")
SUBJUNCTIVE_IMPERFECT=$(subjunctive_imperfect "$word" "$stem" "$ending")
SUBJUNCTIVE_FUTURE=$(subjunctive_future "$word" "$stem" "$ending")
PAST_PARTICIPLE=$(past_participle "$word" "$stem" "$ending")
GERUND=$(gerund "$word" "$stem" "$ending")

echo "${INDICATIVE_PRESENT} ${INDICATIVE_IMPERFECT} ${PRETERITE} ${FUTURE} ${CONDITIONAL} ${SUBJUNCTIVE_PRESENT} ${SUBJUNCTIVE_IMPERFECT} ${SUBJUNCTIVE_FUTURE} ${PAST_PARTICIPLE} ${GERUND}"
}

create_dict()
{
echo "$ENTRIES"| while read line
do
	if [[ ${line: -1} == ">" ]] # if this is true, line is the english entry;
	then 	                      # if not true, then we kow that line must be a singular spanish word
		el="${line//<orth>}"
		echo "<en> ${el//<\/orth>}" >> conjugated_dict.txt
	elif [[ ${line: -2} == "ar" ]] || [[ ${line: -2} == "er" ]] || [[ ${line:-2} == "ir" ]]
	then
		echo "$line $(noun_adj_handler "$line") $(verb_handler "$line")" >> conjugated_dict.txt # we call both in case this encounters a noun ending in -ar -er or -ir
	else
		echo "$line $(noun_adj_handler "$line")" >> conjugated_dict.txt
fi
done

echo "finished creating conjugated_dict.txt!"
}

create_dict
