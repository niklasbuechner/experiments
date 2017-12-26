const ButtonRenderer = {
	config: {
		paddingBorderSVGInPx: 5,
		SVGWidthInOx: 20,
		paddingSVGTextInPx: 5,
		textWidthInPx: 100,
		paddingTextBorderInPx: 5,
		paddingBetweenButtonsInPx: 5,
	},

	items: [
		{ svgColor: 'blue', background: 'red', text: 'Tags' },
		{ svgColor: 'green', background: 'yellow', text: 'Checklist' },
		{ svgColor: 'brown', background: 'orange', text: 'Deadline'}
	],

	computed: {
		minBtnWidthInPx: 0,
		maxBtnWidthInPx: 0,
		svgTotalWidthInPx: 0,
	},

	currentState: [],

	initComputedValues: function () {
		if (this.computed.minBtnWidthInPx !== 0) {
			return ;
		}

		this.computed.minBtnWidthInPx = this.config.paddingBorderSVGInPx + this.config.SVGWidthInOx + this.config.paddingTextBorderInPx;
		this.computed.maxBtnWidthInPx = this.computed.minBtnWidthInPx + this.config.paddingSVGTextInPx + this.config.textWidthInPx;
		this.computed.svgTotalWidthInPx = (this.items.length - 1) * (this.computed.minBtnWidthInPx + this.config.paddingBetweenButtonsInPx) + this.config.paddingBetweenButtonsInPx + this.computed.maxBtnWidthInPx;
	},

	renderButtons: function () {
		this.initComputedValues();

		const minBtnWidthInPx = this.computed.minBtnWidthInPx;
		const svgTotalWidthInPx = this.computed.svgTotalWidthInPx;
		const positions = this.calculateMinButtonPositions();

		let svgHTML = `<svg viewBox="0 0 ${svgTotalWidthInPx} 30" xmlns="http://www.w3.org/2000/svg" width="${svgTotalWidthInPx}" height="30">`;
		let defsHTML = '';
		let buttonsHTML = '';

		let i = 0;

		this.items.forEach((item) => {
			const btnPosition = positions[i];
			const svgPosition = btnPosition + this.config.paddingBorderSVGInPx;
			const textPosition = svgPosition + this.config.SVGWidthInOx + this.config.paddingSVGTextInPx;
			const backgroundRectStart = `<rect x="${btnPosition}" y="0" rx="3" width="${minBtnWidthInPx}" height="30"`;

			defsHTML += `<clipPath id="expandingText${i}">${backgroundRectStart} id="textClip${i}" /></clipPath>`

			buttonsHTML += `<g id="btnGroup${i}">`
			buttonsHTML += `${backgroundRectStart} fill="${item.background}" id="background${i}" />`;
			buttonsHTML += `<rect x="${svgPosition}" y="5" width="20" height="20" fill="${item.svgColor}" />`;
			buttonsHTML += `<text x="${textPosition}" y="22" font-family="Arial" font-size="20" clip-path="url(#expandingText${i})">${item.text}</text>`;
			buttonsHTML += '</g>'

			this.currentState.push({width: minBtnWidthInPx, x: btnPosition});
			i += 1;
		});

		svgHTML += `<defs>${defsHTML}</defs>${buttonsHTML}`;
		svgHTML += '</svg>';

		document.getElementsByTagName('body')[0].innerHTML += svgHTML;

		for (let i = 0; i < this.items.length; i++) {
			document.getElementById(`btnGroup${i}`).addEventListener('click', (event) => {
				this.showButton(event);
			})
		}
	},

	calculateMinButtonPositions: function () {
		const itemAmount = this.items.length;
		const itemPositions = [];

		let i = 0;

		this.items.forEach(() => {
			const amountBtnsAfertThisOne = itemAmount - 1 - i;
			const x = this.computed.svgTotalWidthInPx - (amountBtnsAfertThisOne * (this.computed.minBtnWidthInPx + this.config.paddingBetweenButtonsInPx)) - this.computed.minBtnWidthInPx;

			itemPositions.push(x);
			i += 1;
		});

		return itemPositions;
	},

	showButton: function (event) {
		const currentState = this.currentState; 
		const newState = [];
		let alreadyUsedWidth = 0;

		for (let i = 0; i < this.items.length; i++) {
			let width = 0;

			if (event.currentTarget.id === `btnGroup${i}`) {
				width = this.computed.maxBtnWidthInPx;
			} else {
				width = this.computed.minBtnWidthInPx;
			}

			let x = alreadyUsedWidth;

			if (alreadyUsedWidth === 0) {
				alreadyUsedWidth += this.config.paddingBetweenButtonsInPx;
			}
			alreadyUsedWidth += width + this.config.paddingBetweenButtonsInPx;
			newState.push({ width, x });
		}

		this.animateStateChange(currentState, newState);
	},

	animateStateChange: function (oldState, newState) {
		if (oldState.length != newState.length) {
			console.log('Invalid states');
			return ;
		}

		const animations = [];

		for (let i = 0; i < oldState.length; i++) {
			let fromCSS = '';
			let toCSS = '';

			if (oldState[i].width != newState[i].width) {
				fromCSS += `width: ${oldState[i].width}px;`;
				toCSS += `width: ${newState[i].width}px;`;
			}

			if (oldState[i].x != newState[i].x) {
				fromCSS += `x: ${oldState[i].x}px;`;
				toCSS += `x: ${newState[i].x}px;`;
			}		

			console.log(`@keyframes transitAnimation${i} { from {${fromCSS}} to{${toCSS}} }`);
			animations.push(`@keyframes transitAnimation${i} { from {${fromCSS}} to{${toCSS}} }`);
		}

		document.getElementById('computedStyle').innerHTML = animations.join(' \n');

		Object.keys(this.items).forEach((index) => {
			const element = document.getElementById(`background${index}`);

			element.style.animation = `3s linear transitAnimation${index}`;
			element.addEventListener('animationend', event => {
				event.currentTarget.style.animation = '';
				event.currentTarget.style.width = `${newState[index].width}px`;
				event.currentTarget.style.x = `${newState[index].x}px`;
			});
		});

		this.currentState = newState;
	}
};

window.addEventListener('load', () => {
	ButtonRenderer.renderButtons();
});