# rs-ast

Sandbox for playing with tree structures. Just to get a handle on parsing.
I chose regular expressions as the language to parse, because (a) I'm familiar with them; and (b) I might need to use a modified regex library for building regex-based tokenizers. 

As with so many people, I found a lot of inspiration in Russ Cox's blog posts at
https://swtch.com/~rsc/regexp/.

Ultimately, the goals of a regex-based natural language tokenizer are rather different from the goals of a general regex library. 

* Matching is always constrained to start at a particular string position
   (aka the start of the "remaining characters" slice).
* It may make sense to compile to DFAs off-line, and load the compiled 
   transition tables on start-up. More generally, we expect heavy traffic: all 
   text in a document is expected to match some pattern. This is not search.
* From experience, I'm pretty sure we can do without capture groups.
   Some times you need to break up the matched string in the action code,
   but really it's better to do it there than pay for capture group support 
   everywhere.


