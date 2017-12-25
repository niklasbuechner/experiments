const BUTTON_LEFT_PADDING_IN_PX = 5;
const SVG_SIZE_IN_PX = 12;
const BUTTON_PADDING_SVG_INPUT_IN_PX = 5;
const INPUT_SIZE_IN_PX = 100;
const BUTTON_RIGHT_PADDING_IN_PX = 5;

const PADDING_BETWEEN_BUTTONS_IN_PX = 10;
const BAR_PADDING_RIGHT_IN_PX = 0;

const BUTTON_MAX_WIDTH = BUTTON_LEFT_PADDING_IN_PX + SVG_SIZE_IN_PX + BUTTON_PADDING_SVG_INPUT_IN_PX + INPUT_SIZE_IN_PX + BUTTON_RIGHT_PADDING_IN_PX;
const BUTTON_MIN_WIDTH = BUTTON_LEFT_PADDING_IN_PX + SVG_SIZE_IN_PX + BUTTON_RIGHT_PADDING_IN_PX;

function renderItems(items) {
	const bar = document.getElementById('bar');
	const ids = [];
	let i = 0;

	items.forEach(item => {
		const rightButtonPosition = (items.length - 1 - i) * (BUTTON_MIN_WIDTH + PADDING_BETWEEN_BUTTONS_IN_PX) + BAR_PADDING_RIGHT_IN_PX;
		const rightSVGPosition = rightButtonPosition + BUTTON_RIGHT_PADDING_IN_PX;
		console.log('previousItems: ' + (items.length - 1 - i));
		console.log('position: ' + rightButtonPosition);

		let itemString = `<div id="button${i}" class='child button' `;
		itemString += `style='background: ${item.background}; right: ${rightButtonPosition}px; width: ${BUTTON_MIN_WIDTH}'></div>`;
		
		itemString += `<svg width='12' height='12' viewBox='0 0 10 10' id='toggle${i}' class="child"`
		itemString += `style='right: ${rightSVGPosition}px;top:6px;'>`;
		itemString += `<rect x='0' y='0' width='10' height='10' fill='${item.svgColor}' />`;
		itemString += `</svg>`;

		itemString += `<input type="text" placeholder="${item.text}" id='toggle${i}Input' class='child' `
		itemString += `style='top: 2px;visibility:hidden;display:none;' />`;

		bar.innerHTML += itemString;
		ids.push(`toggle${i}`)

		i += 1;
	});

	ids.forEach((id) => {
		document.getElementById(id).addEventListener('click', showInput);
	});
}

function showInput(event) {
	const buttonId = `button${event.currentTarget.id.substr(6)}`;
	const button = document.getElementById(buttonId);

	const buttonWidthDifference = BUTTON_MAX_WIDTH - BUTTON_MIN_WIDTH;
	const buttonHalfWayWidth = BUTTON_MIN_WIDTH + (buttonWidthDifference / 2);
	button.style.width = `${buttonHalfWayWidth}px`;
	button.style.transform = `translate(${(buttonHalfWayWidth - BUTTON_MIN_WIDTH) / 2}px) scaleX(${BUTTON_MIN_WIDTH / buttonHalfWayWidth})`;

	const svg = event.currentTarget;
	const oldSVGPosition = parseFloat(window.getComputedStyle(svg).getPropertyValue('right').slice(0, -2));
	const newSVGPosition = oldSVGPosition + BUTTON_PADDING_SVG_INPUT_IN_PX + INPUT_SIZE_IN_PX;
	const svgHalfWayPoint = oldSVGPosition + (BUTTON_PADDING_SVG_INPUT_IN_PX + INPUT_SIZE_IN_PX) / 2;
	svg.style.right = `${svgHalfWayPoint}px`;
	svg.style.transform = `translate(${svgHalfWayPoint - oldSVGPosition}px)`;

	const inputId = `toggle${event.currentTarget.id.substr(6)}Input`;
	const input = document.getElementById(inputId);
	const inputFinalPosition = newSVGPosition - BUTTON_PADDING_SVG_INPUT_IN_PX - INPUT_SIZE_IN_PX;
	const inputHalfWayPosition = svgHalfWayPoint - BUTTON_PADDING_SVG_INPUT_IN_PX - INPUT_SIZE_IN_PX;
	input.style.visibility = 'visible';
	input.style.display = 'block';
	input.style.right = `${inputHalfWayPosition}px`;
	input.style.opacity = 0;
	input.focus();

	requestAnimationFrame(() => {
		button.classList.add('animate-transform-to-half-way-point');
		svg.classList.add('animate-transform-to-half-way-point');

		button.style.transform = '';
		svg.style.transform = '';
	});

	const removeAnimateTransformClass = event => {
		event.currentTarget.classList.remove('animate-transform-from-half-way-point');
		event.currentTarget.removeEventListener('transitionend', removeAnimateTransformClass);
	};

	const halfWayPointReached = (event) => {
		svg.classList.remove('animate-transform-to-half-way-point');
		button.classList.remove('animate-transform-to-half-way-point');

		svg.style.right = `${newSVGPosition}px`;
		svg.style.transform = `translate(${newSVGPosition - svgHalfWayPoint}px)`;

		input.style.right = inputFinalPosition;
		input.style.transform = `translate(${inputFinalPosition - inputHalfWayPosition}px)`;

		const buttonFinalWidth = BUTTON_MAX_WIDTH
		button.style.width = `${buttonFinalWidth}px`;
		button.style.transform = `translate(${(buttonFinalWidth / buttonHalfWayWidth) / 2}px) scaleX(${buttonHalfWayWidth / buttonFinalWidth})`;

		requestAnimationFrame(() => {
			svg.classList.add('animate-transform-from-half-way-point');
			svg.style.transform = '';

			input.classList.add('animate-transform-from-half-way-point');
			input.style.transform = '';
			input.style.opacity = 1;

			button.classList.add('animate-transform-from-half-way-point');
			button.style.transform = '';
		});

		input.addEventListener('transitionend', removeAnimateTransformClass);

		if (button.id === event.currentTarget.id) {
			button.addEventListener('transitionend', removeAnimateTransformClass);
			button.removeEventListener('transitionend', halfWayPointReached);
		} else if (svg.id === event.currentTarget.id) {
			svg.addEventListener('transitionend', removeAnimateTransformClass);
			svg.removeEventListener('transitionend', halfWayPointReached);
		}
	}
	button.addEventListener('transitionend', halfWayPointReached);
	svg.addEventListener('transitionend', halfWayPointReached);
}

window.addEventListener('load', () => {
	const items = [
		{ text: 'Tags', svgColor: 'green', background: 'orange' },
		{ text: 'Checklist', svgColor: 'red', background: 'limegreen' },
		{ text: 'Deadline', svgColor: 'purple', background: 'blue' }
	];

	renderItems(items);
});