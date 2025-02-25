# Translations in memsos

The memsos translation system is quite simple, it reads a LANG environment variable which contains the language in which the memsos iso is going to be, then the build reads the lang json (found in defs) and generates a file called lang_info.rs (although I do not recommend reading it directly because it is usually generated in a bad format so its reading by the human eye is usually a little difficult because of that).

# How can i create my own translation?

It doesn't have much magic you can simply copy and paste the structure of one of the existing jsons and simply translate the content of it and it should now follow this structure:

lang_region.UTF-8.json
an example of this is:
en_US.UTF-8.json
