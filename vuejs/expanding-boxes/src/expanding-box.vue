<template>
	<div class="expanding-box" v-on:click="toggleBox">
		<div class="small-text">
			Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas ac eleifend nisi. Aliquam sed risus at mi viverra ullamcorper. Fusce finibus est sed nunc placerat, id tincidunt dolor pretium.
		</div>
		<div class="big-text big-text--hidden">
			Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas ac eleifend nisi. Aliquam sed risus at mi viverra ullamcorper. Fusce finibus est sed nunc placerat, id tincidunt dolor pretium. Proin rhoncus tortor nisi, sed tempus ante convallis id. Aenean gravida efficitur fringilla. Nam ultricies feugiat cursus. Donec luctus, risus at sagittis mollis, ipsum odio tincidunt neque, sed tincidunt mi felis non erat. Pellentesque semper nulla libero, nec maximus ex ornare ac. Praesent ut neque sit amet velit tempor convallis eget sit amet mauris. Ut pulvinar turpis at dolor rutrum, id bibendum velit lacinia. Proin a ornare dolor.
		</div>
		<div class="overlay">
			&nbsp;
		</div>
	</div>
</template>

<script type="text/javascript">
import AnimationMixin from './mixins/Animation.vue';

export default {
	mixins: [AnimationMixin],
	methods: {
		toggleBox() {
			const animatedElements = [];
			let currentSibling = this.$el.nextSibling;

			while (currentSibling) {
				if (currentSibling.nodeType !== Node.TEXT_NODE) {
					animatedElements.push(currentSibling);
				}
				currentSibling = currentSibling.nextSibling;
			}

			const bigText = this.$el.getElementsByClassName('big-text')[0];
			const overlay = this.$el.getElementsByClassName('overlay')[0];

			const alterDOMForAnimationCallback = (elements) => {
				bigText.className = bigText.className.replace(/big-text--hidden/, '');

				const bigTextBoundingBox = bigText.getBoundingClientRect();
			};

			const dimensionChangesDuringAnimation = this.calculateDimensionChanges(animatedElements, alterDOMForAnimationCallback);
			console.log(dimensionChangesDuringAnimation);

			const bigTextBoundingBox = bigText.getBoundingClientRect();
			const parentElementBox = this.$el.getBoundingClientRect();
			overlay.style.visibility = 'visible';
			overlay.style.display = 'block';
			overlay.style.width = `${bigTextBoundingBox.width}px`;
			// overlay.style.height = `${bigTextBoundingBox.height}px !important`;
			// debugger;
			overlay.style.top = `${parentElementBox.height}px`;

			const currentBigTextBoungingRect = bigText.getBoundingClientRect();
			dimensionChangesDuringAnimation.overlay = {
				beforeAnimation: {
					height: currentBigTextBoungingRect.height,
					top: currentBigTextBoungingRect.top - parentElementBox.top,
				},
				afterAnimation: {
					height: 1,
					top: parentElementBox.height - (currentBigTextBoungingRect.height / 2),
				},
			};

			animatedElements.overlay = overlay;

			this.setupAnimation(animatedElements, dimensionChangesDuringAnimation);
			// debugger;
			this.playAnimation(animatedElements)
		},
		prepareEntrance(animatedElement, alterDOMForAnimationCallback) {
			alterDOMForAnimationCallback(animatedElement);
			// const dimensions = this.calculateDimensions(animatedElement, alterDOMForAnimationCallback);
			// this.setupAnimation(animatedElement, dimensions);
			// this.playAnimation(animatedElement, dimensions);
		},
		enter() {

		},
	},
};
</script>

<style>
	.expanding-box, .big-text {
		padding: 10px;
		margin-top: 10px;
	}
	.expanding-box {
		position: relative;
		background-color: white;
		/*color: white;*/
	}
	.big-text {
		/*background-color: red;*/
		color: black;
	}
	.big-text--hidden {
		height: 1px !important;
		overflow: hidden;
		padding: 0px;
	}
	.overlay {
		background: linear-gradient(rgba(0, 0, 0, 0), rgba(255, 255, 255, 1) 20%);
		visibility: hidden;
		position: absolute;
		pointer-events: none;
		height: 1px !important;
		overflow: hidden;
		padding: 0px;
	}
</style>