let progressValue = 0;

function calculateForm(value) {
	const progressCircle = document.getElementById('progressCircle');
	const fullCircle = document.getElementById('fullCircle');

	if (value <= 0) {
		progressCircle.style.visibility = 'hidden';
		fullCircle.style.visibility = 'hidden';

		return;
	}

	if (value >= 1) {
		progressCircle.style.visibility = 'hidden';
		fullCircle.style.visibility = 'visible';

		return;
	}

	progressCircle.style.visibility = 'visible';
	fullCircle.style.visibility = 'hidden';

	// x = center x + cos (angle) * radius
	const centerX = 50;
	const centerY = 50;
	const radius = 40;
	const radiusX = radius;
	const radiusY = radius;
	const angleInRadian = Math.PI * 2 * value;

	const finalX = centerX + radius * Math.cos(angleInRadian);
	const finalY = centerY + radius * Math.sin(angleInRadian);

	const largeArcFlag = value > .5 ? 1 : 0;

	let pathString = 'M50, 50 '; // Move to 50 50
	pathString += 'l40, 0 '; // Line to 0 (x), -40 (y)
	pathString += 'A' + radiusX + ' ' + radiusY + ' 1 ' + largeArcFlag + ' 1 ' + finalX + ' ' + finalY;

	console.log('value: ' + value);
	console.log('x: ' + finalX);
	console.log('y: ' + finalY);
	console.log('path: ' + pathString);

	progressCircle.setAttribute('d', pathString);
}

function animateProgress(newValue) {
	const animationTimeForAFullCircleInMilliseconds = 1000;
	const oldValue = progressValue;
	const valueDifference = newValue - oldValue;
	const animationTimeInMilliseconds = Math.abs(animationTimeForAFullCircleInMilliseconds * valueDifference);
	const animationStartTimeStamp = window.performance.now();

	const calculateNextStep = () => {
		const currentTimeStamp = window.performance.now();
		const timeDifference = currentTimeStamp - animationStartTimeStamp;

		const currentValue = timeDifference / animationTimeInMilliseconds * valueDifference + oldValue;

		if (timeDifference < animationTimeInMilliseconds && currentValue != newValue) {
			calculateForm(currentValue);
			progressValue = currentValue;

			requestAnimationFrame(calculateNextStep);
		} else {
			progressValue = newValue;
			calculateForm(newValue);
		}
	};

	requestAnimationFrame(calculateNextStep);
}

window.addEventListener('load', () => {
	document.getElementById('progression').addEventListener('change', () => {
		const value = parseFloat(document.getElementById('progression').value);
		animateProgress(value / 100);
	});
	progressValue = parseFloat(document.getElementById('progression').value / 100);
	calculateForm(progressValue);
});