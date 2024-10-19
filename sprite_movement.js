(async function () {
    const MAX_FRAME = 30;
    DEFAULT_TIMEOUT = 32;

    const boardEle = document.querySelector("#board");
    const groupEles = Array.from(document.querySelectorAll(".group"));

    console.log("Starting loops");

    const movementFuncs = [
        clockwiseToRightLoop,
        moveUpAndDownLoop,
        counterClockwiseToRightLoop,
    ];

    for (let i = 0; i < groupEles.length; i++) {
        (async function () {
            const ele = groupEles[i];

            const j = randIntBetween(0, movementFuncs.length - 1);

            let frame = 0;
            let incr = true;

            while (true) {
                ele.style.top = `${frame * 4 * (i + 1)}px`;

                incr ? frame++ : frame--;
                if (frame === MAX_FRAME) {
                    incr = false;
                } else if (frame === 0) {
                    incr = true;
                }

                await wait(30);
            }
        })();

        // if (i < 2) {
        //     clockwiseToRightLoop(ele);
        // } else if (i < 5) {
        //     moveUpAndDownLoop(ele);
        // } else {
        //     counterClockwiseToRightLoop(ele);
        // }
    }

    async function counterClockwiseToRightLoop(element, timeout = DEFAULT_TIMEOUT, radius = 30) {
        let deg = 0;

        while (true) {
            const x = radius * Math.cos(deg);
            const y = radius * Math.sin(deg);

            element.style.transform = `translate(${-x}px,${-y}px)`;

            deg >= 360 ? deg = 0 : deg -= 0.1;

            await wait(timeout);
        }
    }

    async function counterClockwiseToLeftLoop(element, timeout = DEFAULT_TIMEOUT, radius = 30) {
        let deg = 0;

        while (true) {
            const x = radius * Math.cos(deg);
            const y = radius * Math.sin(deg);

            element.style.transform = `translate(${x}px,${y}px)`;

            deg >= 360 ? deg = 0 : deg -= 0.1;

            await wait(timeout);
        }
    }

    async function clockwiseToLeftLoop(element, timeout = DEFAULT_TIMEOUT, radius = 30) {
        let deg = 0;

        while (true) {
            const x = radius * Math.cos(deg);
            const y = radius * Math.sin(deg);

            element.style.transform = `translate(${-x}px,${-y}px)`;

            deg >= 360 ? deg = 0 : deg += 0.1;

            await wait(timeout);
        }
    }

    async function clockwiseToRightLoop(element, timeout = DEFAULT_TIMEOUT, radius = 30) {
        let deg = 0;

        while (true) {
            const x = radius * Math.cos(deg);
            const y = radius * Math.sin(deg);

            element.style.transform = `translate(${x}px,${y}px)`;

            deg >= 360 ? deg = 0 : deg += 0.1;

            await wait(timeout);
        }
    }

    async function spinClockwiseLoop(element, timeout = DEFAULT_TIMEOUT) {
        let deg = 0;

        while (true) {
            element.style.transform = `rotate(${deg}deg)`;
            deg >= 360 ? deg = 0 : deg++;

            await wait(timeout);
        }
    }

    async function spinCounterClockwiseLoop(element, timeout = DEFAULT_TIMEOUT) {
        let deg = 0;

        while (true) {
            element.style.transform = `rotate(${deg}deg)`;
            deg >= 360 ? deg = 0 : deg--;

            await wait(timeout);
        }
    }

    async function moveUpAndDownLoop(element, timeout = DEFAULT_TIMEOUT) {
        let frame = 0;
        let incr = true;

        while (true) {
            element.style.top = `${frame}px`;

            incr ? frame++ : frame--;
            if (frame === MAX_FRAME) {
                incr = false;
            } else if (frame === 0) {
                incr = true;
            }

            await wait(timeout);
        }
    }

    async function wait(timeout) {
        return new Promise(resolve => setTimeout(resolve, timeout));
    }

    function degToRad(deg) {
        return deg * Math.PI / 180;
    }

    function randIntBetween(min, max) {
        min = Math.ceil(min);
        max = Math.floor(max);
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }
})();
