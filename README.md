# YAM
*"YAM Ain't yo Mama"*

---
Things I've said about YAM:

> A new way to store and use notes

> A Totally Tubular Note-Keeping Tuber
---

YAM is a note storage solution inspired by years of using
[Obsidian](https://obsidian.md) and finding that I lacked the 
capacity to hold myself to any standard while creating and 
updating notes. Where Obsidian makes no demands, YAM requires 
a strict adherence to its rules.

What you need to know about YAM:

- All notes are stored in plaintext files, a mixture of Markdown 
  and YAML (hence the name).
- There are three classes of data stored in the vault: Notes, 
  Information, and Syntheses.
- Notes and Syntheses are Markdown files, Information are YAML 
  files.
- Notes are where you collect knowledge.
- Information are where you distill that knowledge into atomic 
  YAML structures (not as complex as it sounds).
- Syntheses are where you synthesize new arguments and 
  information from pieces of existing Information.
- Templates are used to define the YAML structures used in 
  Information for quick reuse of common structures.

## Installation

TODO

## How does it all work?

You probably already have a glaring question: *Obsidian has YAML, 
why not just use Obsidian?* You can! I, however, am bad at making 
myself keep to standards when I am quickly grabbing pieces of 
information and adding them to my vault. I need a system that 
will force me to stop, atomize, and link, otherwise I simply 
won't, and Obsidian is too focused on being to be a 
one-tool-fits-all application for note taking and storage (not 
a bad thing! I love Obsidian enough to try and copy what it's 
doing), and I need something more specific.

If you are still interested, you're probably also thinking: 
*Alright, this is interesting, but what are Notes, Information, 
and Syntheses? How do they work together? And what's YAML?* Great question!

### YAML

Let's start with the easy one first: *YAML* is a structured 
data serialization format similar to JSON (just a fancy way of 
saying we can create a structure of data that describes how it's 
structured), but it's considered "human readable". Here is 
a quick example of some YAML:

```YAML
# this is a YAML comment
name: YAML_File
# This is a YAML object that has multiple associated keys and
# values within it
attributes:
    is_cool: true
    is_json: false
    description: "A human-readable data structure stored in plain text"
# here we have a simple list; we can call it whatever we want,
# but we chose a_list
a_list:
    - first_element # this is a string with no spaces; there is no colon here, so it's assumed to be a string
    - "second element" # this is a string with spaces
    - 3 # this is a number
# here we have another list with YAML objects as elements
another_list:
    - type: string # we can also have a comment here
      desc: "An element in a list"
      value: a_list_value
    - type: number
      desc: "A number held in a list"
      value: 42
```

It's relatively easy to read, and plenty of information about it 
is on the internet. Everything *before* the colon (:) is a "key", 
or what we can use to look up information in the YAML data. The 
value *after* the colon is the "value", what we get when we look 
up a specific key in the YAML data. Once you get the hang of it 
it's easy to use.

### Notes, Information, and Syntheses

If you've ever used Obsidian or are familiar with the 
Zettelkasten method, then you'll be familiar with how notes are 
taken, stored, and used there. If not, here's a quick run-down:
  
---
Anytime you read or learn something new, you create 
**literature notes** to store the information. The goal is to get 
the note to be succinct and discuss only one thing. So, if you 
read an article discussing the different types of nuclear 
radiation, you could have a literature note for each of alpha, 
beta, and gamma particles, along with what can cause them, what 
they're made of, how much energy each has, etc.

From here, you would go through your existing notes and find 
other notes that are related to the radiation particle literature 
notes. Obviously they are related to each other, so you could 
create another note overviewing nuclear radiation types, and all 
three cards would be linked to that card and vice-versa. Later, 
you could create a literature note about some radioactive decay,
like Uranium-238, which decays into an alpha particle and 
Thorium-234. This literature note would be connected back to the 
alpha particle literature note from before.

There are also **fleeting notes**, which hold ideas or concepts 
that you mihgt come up with throughout the day. Generally, they 
are supposed to be gone through daily to weed out the stuff you 
don't want to keep (most of it) and keep the stuff you do want to 
keep, turning them into permanent notes.

Finally, **permanent notes** are a larger category that contain 
**literature notes**, any **fleeting notes** we want to keep, and 
notes that are created in the process of joining multiple 
**literature notes** together. These stay in the vault and are 
the notes that you go to to retreive information.

If you want to learn more, I would highly suggest reading posts 
from [zettelkasten.de](https://zettelkasten.de/).

---

How do YAM's style of notes fit into this system? It's pretty 
simple:

<dl>
    <dt>Notes are similar to fleeting notes</dt>
    <dd>
        Notes are exactly what they sound like: when you are 
        reading, watching, or in general learning (or having ideas), 
        you write them in Markdown format however you normally 
        would. These are for you to record your thoughts and what 
        you're learning so you can organize it later.
    </dd>
    <dt>Information are similar to literature notes</dt>
    <dd>
        Information are the YAML files we keep that have atomized 
        pieces of information. When we have a Note file, we take 
        it and break it down into one or more Information files 
        that contain the atomized concepts and ideas from our 
        Notes. This is, in generaly, what literature notes are.
    </dd>
    <dt>Syntheses are similar to permanent notes</dt>
    <dd>
        Syntheses are the collection of multiple 
        literature/permanent notes to synthesize new 
        information. This synthesis could look like the chapter 
        from a book, or an article, or an essay, or anything that 
        could be considered a finish work that cites sources and 
        reconveys existing information in a new light. These are 
        the actual work that you would produce from doing 
        research.
    </dd>
<dl>

If you understand how Obsidian and the Zettelkasten method work, 
then it should be easy to understand how YAM works: the only 
difference is, YAM forces you to stick to this method.

### Templates

Templates are a description of the YAML that is used for an 
Information file. They tell YAM how a certain Information file is 
supposed to look so that when you need to add Information to the 
vault, you can quickly pull up the template and fill out the 
file.

YAML and Templates are great because not only do they help us 
keep information in a consistent format, it's also easier to 
search through everything. What if you want a list of all the
amino acids you have recorded that are used in a select set of 
proteins, which are also used in certain types of biological 
processes? You could try to search text files for that 
information, but it would be tough. How do you structure that 
result? You can't, unless you use some structure already. With 
YAML and Templates, we *know* that all of these Information will 
have certain structures and certain types of relationships, so we 
only have to describe what we are looking for in the structures 
and what types of relationships we are looking for. You'll know 
you're getting the right information because it's been structured 
that way.

> Templates are not yet implemented in YAM, see the roadmap for 
> when they are anticipated to be implemented into the system.

## Roadmap

Right now, YAM is currently under ***active development***, and 
a final, stable version has yet to be released. Below is 
a roadmap of features that are planned to be implemented:

- [ ] Templates
- [ ] Graph Database
- [ ] File Manager
- [ ] Cross-Platform GUI

More will be added as the project progresses, so check in 
occassionaly. If you don't see something you want on here, create 
an issue so it can be discussed.
