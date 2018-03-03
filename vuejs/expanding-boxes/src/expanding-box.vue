<template>
	<div class="expanding-box" v-on:click="toggleBox">
		<div class="small-text">
			Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas ac eleifend nisi. Aliquam sed risus at mi viverra ullamcorper. Fusce finibus est sed nunc placerat, id tincidunt dolor pretium.
		</div>
		<transition
			v-on:before-enter="prepareEntrance"
			v-on:enter="enter"
		>
			<div class="big-text big-text--hidden">
				Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas ac eleifend nisi. Aliquam sed risus at mi viverra ullamcorper. Fusce finibus est sed nunc placerat, id tincidunt dolor pretium. Proin rhoncus tortor nisi, sed tempus ante convallis id. Aenean gravida efficitur fringilla. Nam ultricies feugiat cursus. Donec luctus, risus at sagittis mollis, ipsum odio tincidunt neque, sed tincidunt mi felis non erat. Pellentesque semper nulla libero, nec maximus ex ornare ac. Praesent ut neque sit amet velit tempor convallis eget sit amet mauris. Ut pulvinar turpis at dolor rutrum, id bibendum velit lacinia. Proin a ornare dolor.
			</div>
		</transition>
	</div>
</template>

<script type="text/javascript">
export default {
	methods: {
		toggleBox() {
			const bigText = this.$el.getElementsByClassName('big-text')[0];

			this.prepareEntrance(bigText, (animatedElement) => {
				animatedElement.className = animatedElement.className.replace(/big-text--hidden/, '');
			});
		},
		prepareEntrance(animatedElement, alterDOMForAnimationCallback) {
			const dimensions = this.calculateDimensions(animatedElement, alterDOMForAnimationCallback);
			this.setupAnimation(animatedElement, dimensions);
			this.playAnimation(animatedElement, dimensions);
		},
		enter() {

		},
		calculateDimensions(animatedElement, alterDOMForAnimationCallback) {
			const beforeBoundingRectangle = animatedElement.getBoundingClientRect();
			const dimensions = {
				beforeEntrance: {
					height: beforeBoundingRectangle.height,
				},
				afterEntrance: {},
			};

			alterDOMForAnimationCallback(animatedElement);

			const afterBoundingRectangle = animatedElement.getBoundingClientRect();
			dimensions.afterEntrance = {
				height: afterBoundingRectangle.height,
			};

			console.log(dimensions);
			return dimensions;
		},
		getPixelValue(valueString) {
			if (valueString.substring(valueString.length - 2) !== 'px') {
				return valueString;
			}

			return valueString.substring(0, valueString.length -2);
		},
		setupAnimation(animatedElement, dimensions) {
			const animationProperties = [];
			if (dimensions.beforeEntrance.height !== dimensions.afterEntrance.height) {
				const scaleYFactor = dimensions.beforeEntrance.height / dimensions.afterEntrance.height;
				const translateTopPosition = (dimensions.beforeEntrance.height - dimensions.afterEntrance.height) / 2;
				animationProperties.push(`translateY(${translateTopPosition}px) scaleY(${scaleYFactor})`);
			}

			animatedElement.style.transform = animationProperties.join(' ');
		},
		playAnimation(animatedElement, dimensions) {
			requestAnimationFrame(() => {
                requestAnimationFrame(() => {
                	if (dimensions.beforeEntrance.height !== dimensions.afterEntrance.height) {
                		animatedElement.className += 'transition-helper';
						animatedElement.style.transform = '';
					}
                });
            });
		}
	},
};
</script>

<style>
	.expanding-box, .big-text {
		padding: 10px;
		margin-top: 10px;
	}
	.expanding-box {
		background-color: blue;
		color: white;
	}
	.big-text {
		background-color: red;
		color: black;
	}
	.big-text--hidden {
		height: 1px !important;
		overflow: hidden;
		padding: 0px;
	}

	.transition-helper {
		transition: 3s ease;
	}
</style>