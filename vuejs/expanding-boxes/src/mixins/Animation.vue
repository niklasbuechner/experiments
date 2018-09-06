<script type="text/javascript">
export default {
	methods: {
		calculateDimensionChanges(animatedElements, alterDOMForAnimationCallback, dimensionChanges) {
			dimensionChanges = dimensionChanges ? dimensionChanges : {};
			alterDOMForAnimationCallback = alterDOMForAnimationCallback ? alterDOMForAnimationCallback : () => {};
			const elementNames = Object.keys(animatedElements);

			elementNames.forEach((name) => {
				dimensionChanges[name] = dimensionChanges[name] ? dimensionChanges[name] : {};
				dimensionChanges[name].beforeAnimation = this.calculateDimensions(animatedElements[name]);
			});

			alterDOMForAnimationCallback(animatedElements);

			elementNames.forEach((name) => {
				dimensionChanges[name].afterAnimation = this.calculateDimensions(animatedElements[name]);
			});

			return dimensionChanges;
		},
		calculateDimensions(element) {
			const boundingRectangle = element.getBoundingClientRect();
			return {
				height: boundingRectangle.height,
				top: boundingRectangle.top,
			};
		},
		setupAnimation(animatedElements, dimensionChanges) {
			const elementNames = Object.keys(animatedElements)
			console.log(elementNames);
			elementNames.forEach((name) => {
				this.setFirstLastTransform(animatedElements[name], dimensionChanges[name]);
			});
		},
		setFirstLastTransform(animatedElement, dimensions) {
			const animationProperties = [];
			if (dimensions.beforeAnimation.top !== dimensions.afterAnimation.top) {
				const translateTopPosition = dimensions.beforeAnimation.top - dimensions.afterAnimation.top;
				animationProperties.push(`translateY(${translateTopPosition}px)`);
			}

			if (dimensions.beforeAnimation.height !== dimensions.afterAnimation.height) {
				const scaleYFactor = dimensions.beforeAnimation.height / dimensions.afterAnimation.height;
				dimensions.beforeAnimation.top -= (dimensions.beforeAnimation.height - dimensions.afterAnimation.height) / 2
				animationProperties.push(`scaleY(${scaleYFactor})`);
			}

			console.log(animationProperties.join(' '));
			animatedElement.style.transform = animationProperties.join(' ');
		},
		playAnimation(animatedElements) {
			debugger;
			requestAnimationFrame(() => {
                requestAnimationFrame(() => {
                	// debugger;
                	const animatedElementNames = Object.keys(animatedElements);
                	animatedElementNames.forEach((name) => {
                		const element = animatedElements[name];
						element.className += ' transition-helper';
						element.style.transform = '';
                	});
                });
            });
		}
	}
}
</script>

<style type="text/css">
	.transition-helper {
		transition: 3s ease;
	}
</style>