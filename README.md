# dominoes_mexTrain_rust

## Components (described in dominoe and computer vernacular)

### A Dominoe
1. Has Hi and a Lo (type u32)
2. Has a Head and a Tail 
3. Has an orientation (bool, if true head is hi else head is lo, tail follows)
4. toggle_orientation()
5. get_head() returns hi if orientation true
6. get_tail() returns lo if orientation true 

### A Dominoe Deck (bonepile)
1. Starts with 91 tiles in Random order (stays that order for the game)
2. is_empty() returns bool
3. pull_dominoe() returns a Dominoe or None if empty
4. pull_double() returns a Dominoe that is double or None if no doubles or empty


### A hand
1. add_dominoe(Dominoe)()
2. is_empty() returns bool
3. shift(from,to) from/to are u32, moves Dominoe from position 'from' to position 'to' within the hand.
4. toggle_orientation(position) toggle the orientation of Dominoe in hand at position
5. show() displays the Dominoes in position order head to tail

### A train
1. Has a game_double.
2. Has a tail
3. Has a token (bool, if true token is up, else down, init's down)
2. set_game_double() 
3. add(Dominoe) returns bool if successful
	* First Dominoe head must match game-double
	* Subsequent add's: the head of dominoe-to-be-added must match train tail
	* Train tail becomes tail of successfully added Dominoe
4. token_up(),token_down()
5. is_token_up()
6. show() displays the Dominoes in position order head to tail


### A player
1. Has a train
2. Has a hand
3. get_train() so other players can see token or tail
4. take_turn(game)
	1. print out current player number
	2. print out current hand
	3. print out game double
        4. print current player tail ("None" if tail empty)
	5. print all player up-tokens, print spare token ("None" if no spare train yet)
	5. print all tails of hands with tokens up
	6. print lengths of all hands and deck
	7. print take_turn menu (return false means take_turn not done)
		1. move dominoe within hand, to/from
		2. toggle orientation tile # in current hand
		3. can-add-train:ur own, player-train-with-up-token, spare-train if not it's not NONE
		4. mv from current hand to can-add-train 
		5. if cant do step 4, pull from game deck
		6. mv from current hand to can-add-train or put up token 

### A game
1. Has a bonepile
2. Has 4 regular players
3. Has 1 spare player (spare train)
4. spare player's train's token is always up
5. run player.take_turn()'s until end

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
		
### To Do list
3. DominoePlayer set_game_double() needs to
   1. self.train.set_game_double() 


![Optional Text](rust_objects_in_game.jpg)




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
