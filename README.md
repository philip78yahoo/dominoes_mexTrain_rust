# dominoes_mexTrain_rust

## Components (described in dominoe and computer vernacular)

### A Dominoe
1. Has Hi and a Lo (type u32)
2. Has a Head and a Tail 
3. Has an orientation (bool, if true head is hi else head is lo, tail follows)
4. ToggleOrientation()

### A Bonepile
1. Starts with 91 tiles in Random order (stays that order for the game)
2. isEmpty() returns bool
3. pullFrom() returns a Dominoe or None if empty


### A hand
1. addDominoe(Dominoe)()
2. isEmpty() returns bool
3. move(from,to) from/to are u32, moves Dominoe from position 'from' to position 'to' within the hand.
4. toggleOrientation(position) toggle the orientation of Dominoe in hand at position
5. show() displays the Dominoes in position order head to tail

### A train
1. Has a game-double.
2. Has a tail
3. Has a token (bool, if true token is up, else down, init's down)
2. SetGameDouble() 
3. addDominoe(Dominoe) returns bool if successful
	* First Dominoe head must match game-double
	* Subsequent add's: the head of dominoe-to-be-added must match train tail
	* Train tail becomes tail of successfully added Dominoe
4. tokenup(),tokendown()
5. show() displays the Dominoes in position order head to tail 

### A game
1. Has a bonepile
2. Has 4 trains (0..3)
3. Has 4 hands (0..3)
4. Has a spare-train
5. run turns until end

### Idea of the game
1. Bonepile created, game-double found
2. Create 4 hands
3. Create 4 trains, set their game-double to the game-double
4. Run turns until a hand is empty or bonepile empty and all tokens up.
5. A turn...
	1. Count 0..4,0..4 ... til end
	2. Show player's hand
	3. let player...
		1. move/orient tiles within hand or ...
		2. attempt tile from hand to his train or ...
		3. attempt tile from hand to spare train or ...
		4. attempt tile from hand to another train (only if other train token up)
		5. pass tokenup
		6. successful add to trains of unmatched double will tokenup ( a freeze )
		













# =============== Markdown Cheats below =================

# My Heading
## My Sub-heading
### My Sub-sub-heading
My regular paragraphs

A dotted list
- one
- two
- three

A bullet list
* one
* two
* three

A numbered list
1. one
2. two
3. three

 	
~~~~
This is a 
piece of code 
in a block
~~~~

## Definition List (of 2) ...
WordPress
:  A semantic personal publishing platform 

Markdown
:  Text-to-HTML conversion tool

 	
### Example Link (Phil's CNN if you hover)
A [link](http://cnn.com "Phil's CNN").


### Mixed List
1. Item
2. Item
   * Mixed
   * Mixed  
3. Item

Dash, no space, regular paragraph
-4