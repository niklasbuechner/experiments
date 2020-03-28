# My experiments repository
This repository contains some of my programming experiments.
(None of these experiments claim to be fully functioning code snippets but rather an adventure into how the given
function could be implemented.)

## Algorithms

### Base64
A test implementation of base64 for Strings only according to the spec:
[https://tools.ietf.org/html/rfc4648](https://tools.ietf.org/html/rfc4648)

The implementation is in the `base64` folder.

## Editor content management
In order to learn how text editors manage the actual text input in memory, I implemented two possible algorithms
for it.

### Gap algorithm
The first one is the gap algorithm. Here the text editor has an array with the text and some empty fields in the
array -> the gap. This gap will stay at the cursor position. The text editor will then insert the new characters
into the empty fields.

E.g.: Text: `ac` - Cursor after `a`
```
[0] => 'a'
[1] => ''
[2] => ''
[3] => ''
[4] => 'c'
```

Insert `b`
```
[0] => 'a'
[1] => 'b'
[2] => ''
[3] => ''
[4] => 'c'
```

Move the cursor after `c`. (Gap needs to be moved.)
```
[0] => 'a'
[1] => 'b'
[2] => 'c'
[3] => ''
[4] => ''
```

This algorithm can be found in `editor-content-management/gap` and is implemented in Rust.

### Piece table algorithm
The second algorithm is the piece table algorithm. With this algorithm the text editor has a long array with all the
text and only appends to the end of the array. Additionaly, there is a second data structure which defines in which
order the text is supposed to be displayed. This leads to the text being displayed correctly.

E.g.: Text: `ac` - Cursor after `a`
```
[0] => 'a'
[1] => 'c'
[2] => ''
[3] => ''
[4] => ''
```

Text definition: [
	0 - 1,
]


Insert `b`
```
[0] => 'a'
[1] => 'c'
[2] => 'b'
[3] => ''
[4] => ''
```

Text definition: [
	0 - 0,
	2 - 2,
	1 - 1,
]

This algorithm can be found in `editor-content-management/piece-table` and is implemented in Rust.

## Animations

### Expanding input boxes
This html page is an experiment to display an icon - here a colored square - which expands once it is clicked. Thereby
it reveals the text input.

There is an html version of this animation in `html/expandingTextInput.html` and a svg version in
`svg/expandingTextInput.html`.

### Loading icon
This is an animated svg image which can be used as a loading icon. The file is under `svg/loader.html`.

### Progression pie chart
This pie chart is a progress bar in form of a pie chart. It will animate all state transitions. The file can be
found in `svg/progression.html`

### Animated expanding divs
The last experiment has some html divs which expand when they are clicked on. This experiment is implemented
with vuejs and can be found under `vuejs/expanding-boxes`. All installation an run instruction are in the experiment's
README. 
